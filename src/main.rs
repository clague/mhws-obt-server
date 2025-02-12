#![feature(random)]
use std::{net::SocketAddr, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::{command, parser::ValueSource, value_parser, Arg, Command};
use log::{info, warn};
use compio::{fs::File, io::AsyncReadAtExt};
use error::ObtError;
use rustls::{pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer}, ServerConfig};
use ntex::{chain, fn_service, service::{fn_factory_with_config, fn_shutdown, Service}, web};
use futures::{future::ready, TryFutureExt};
use env_logger::Env;

mod error;
mod route;
mod strings;

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

    let on_shutdown = fn_shutdown(|| {
        warn!("Websocket connection shutdown");
    });

    // pipe our service with on_shutdown callback
    Ok(chain(service).and_then(on_shutdown))
}

/// do websocket handshake and start web sockets service
async fn ws_index(req: web::HttpRequest) -> Result<web::HttpResponse, ObtError> {
    //println!("serve /ws");
    web::ws::start::<_, _, ObtError>(req, fn_factory_with_config(ws_service)).await
}

#[ntex::main]
async fn main() -> Result<()> {
    let matches = command!()
        .override_help(strings::HELP.replace("{command}", &std::env::args().nth(0).unwrap_or("mhws-obt-server".to_owned())))
        .arg(Arg::new("crt")
            .short('c').long("crt").value_name("FILE")
            .help("Sets a custom certificate pem file")
            .required(false)
            .default_value("./obt-wilds.crt")
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(Arg::new("key")
            .short('k').long("key").value_name("FILE")
            .help("Sets a custom key pem file")
            .required(false)
            .default_value("./obt-wilds.key")
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(Arg::new("listen")
            .short('l').long("listen").value_name("ADDRESS:PORT")
            .help("Sets a custom key pem file")
            .required(false)
            .default_value("127.0.0.1:443")
            .value_parser(value_parser!(SocketAddr)),
        )
        .get_matches();
    //let args: Vec<String> = std::env::args().collect();
    //OpenOptions::new().write(true).create(true).open("./args.txt").await?.write_all_at(args.join("\n"), 0).await.unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let use_tls = matches.value_source("crt").is_some_and(|s| s != ValueSource::DefaultValue) ||
        matches.value_source("key").is_some_and(|s| s != ValueSource::DefaultValue);

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
        let cert_chain = ready(Ok(File::open(matches.get_one::<PathBuf>("crt").unwrap())
            .await?))
            .and_then(async |f: File| {
                Ok(vec![CertificateDer::from_pem_slice(&f.read_to_end_at(Vec::with_capacity(8192), 0).await.1)?])
            }).await
            .map_err(|e: anyhow::Error| anyhow!("Failed to import certificate: ".to_owned() + &e.to_string()))?;
        let key_der = ready(Ok(File::open(matches.get_one::<PathBuf>("key").unwrap()).await?))
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
        Ok(server().bind_rustls(matches.get_one::<SocketAddr>("listen").unwrap(), tls_config)?.run().await?)
    }).or_else(async |e| {
        if use_tls {
            return Err(e)
        }
        Ok(server().bind(matches.get_one::<SocketAddr>("listen").unwrap())?.run().await?)
    }).await
}