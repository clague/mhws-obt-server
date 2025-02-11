#![feature(random)]
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use log::info;
use compio::{fs::File, io::AsyncReadAtExt};
use error::ObtError;
use clap::Parser;
use rustls::{pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer}, ServerConfig};
use ntex::{fn_service, chain, web, service::{fn_factory_with_config, Service}};
use futures::{future::ready, TryFutureExt};
use env_logger::Env;

mod error;
mod route;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "CRT_FILE")]
    crt: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "KEY_FILE")]
    key: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, value_name = "address:port")]
    listen: Option<String>
}

async fn default_handle() -> Result<&'static str, ObtError> {
    Err(ObtError::new("resources cannot be found", 404).unwrap())
}

async fn ok() -> Result<&'static str, ObtError> {
    Ok("")
}

/// WebSockets service factory
async fn ws_service(
    _sink: web::ws::WsSink,
) -> Result<
    impl Service<web::ws::Frame, Response = Option<web::ws::Message>, Error = ObtError>,
    ObtError,
> {

    // handler service for incoming websockets frames
    let service = fn_service(move |frame| {
        let item = match frame {
            // update heartbeat
            web::ws::Frame::Ping(msg) => {
                Some(web::ws::Message::Pong(msg))
            }
            // update heartbeat
            web::ws::Frame::Pong(_) => {
                None
            }
            // close connection
            web::ws::Frame::Close(reason) => Some(web::ws::Message::Close(reason)),
            // ignore other frames
            web::ws::Frame::Binary(x) | web::ws::Frame::Text(x) => {
                info!("Websocket peer send: {:02X?}", x.to_vec());
                None
            }
            _ => None
        };
        ready(Ok(item))
    });

    // pipe our service with on_shutdown callback
    Ok(chain(service))
}

/// do websocket handshake and start web sockets service
async fn ws_index(req: web::HttpRequest) -> Result<web::HttpResponse, ObtError> {
    //println!("serve /ws");
    web::ws::start::<_, _, ObtError>(req, fn_factory_with_config(ws_service)).await
}

#[ntex::main]
async fn main() -> Result<()> {
    //let args: Vec<String> = std::env::args().collect();
    //OpenOptions::new().write(true).create(true).open("./args.txt").await?.write_all_at(args.join("\n"), 0).await.unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let cli = Cli::parse();
    let use_tls = cli.crt.is_some() || cli.key.is_some();

    let server = || web::HttpServer::new(|| {
        web::App::new()
            .wrap(web::middleware::Logger::default())
            //.wrap(crate::middleware::WsAddHeader)
            .service(route::hjm::system_json)
            .service(route::hjm::analysis)
            .service(route::api::auth_login)
            .service(route::api::delivery_data)
            .service(route::api::hunter_sync)
            .service(route::api::hunter_delete)
            .service(route::api::hunter_profile_update)
            .service(route::api::obt_play)
            .service(route::api::hunter_character_creation_upload)
            .service(route::api::auth_ticket)
            .service(route::api::follow_total_list)
            .service(route::api::offline_notification_list)
            .service(route::api::community_invitation_received_list)
            .service(route::api::block_list)
            .service(route::api::friend_list)
            .service(route::mtm::steam_sign)
            .service(route::mtm::consent_countries)
            .service(route::mtm::consent_restrictions)
            .service(route::mtm::consent_documents)
            .service(route::mtm::token_refresh)
            .service(route::pubsub::earth_analysis_obt)
            .service(route::playfabapi::list_servers)
            .service(web::resource("/200").to(ok))
            .service(web::resource("/ws").route(web::get().to(ws_index)))
            .default_service(web::route().to(default_handle))
    });

    let cert_result: Result<(Vec<CertificateDer>, PrivateKeyDer)> = (async || {
        let cert_chain = ready(Ok(File::open(cli.crt.unwrap_or("./obt-wilds.crt".into()))
            .await?))
            .and_then(async |f: File| {
                Ok(vec![CertificateDer::from_pem_slice(&f.read_to_end_at(Vec::with_capacity(8192), 0).await.1)?])
            }).await
            .map_err(|e: anyhow::Error| anyhow!("Failed to import certificate: ".to_owned() + &e.to_string()))?;
        let key_der = ready(Ok(File::open(cli.key.unwrap_or("./obt-wilds.key".into())).await?))
            .and_then(async |f: File| {
                Ok(PrivateKeyDer::from_pem_slice(&f.read_to_end_at(Vec::with_capacity(8192), 0).await.1)?)
            }).await
            .map_err(|e: anyhow::Error| anyhow!("Failed to import key: ".to_owned() + &e.to_string()))?;

        Ok((cert_chain, key_der))
    })().await;

    ready(cert_result).and_then(async |(cert_chain, key_der)| {
        let tls_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key_der)?;
        Ok(server().bind_rustls(("localhost", 443), tls_config)?.run().await?)
    }).or_else(async |e| {
        if use_tls {
            return Err(e)
        }
        Ok(server().bind(("localhost", 443))?.run().await?)
    }).await
}