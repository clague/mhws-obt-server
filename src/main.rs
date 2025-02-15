#![feature(try_blocks)]
use std::{env::{current_dir, set_current_dir}, net::SocketAddr, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::{builder::ArgPredicate, command, value_parser, Arg, ArgAction};
use compio::{fs::File, io::AsyncReadAtExt};
use error::ObtError;
use rustls::{pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer}, ServerConfig};
use ntex::{http::{header, StatusCode}, web};
use futures::{future::ready, TryFutureExt};
use env_logger::Env;
use base64::prelude::*;

mod error;
mod ws;
mod route;

async fn default_handle(req: web::HttpRequest) -> Result<impl web::Responder, ObtError> {
    let path = req.path()
        .trim_start_matches('/')
        .trim_end_matches('/')
        .to_owned()
        .replace(&[':', '\\', '\"', '*', '?', '<', '>', '|'], "")
        .parse::<PathBuf>()?
        .canonicalize()?;
    if current_dir()?.canonicalize()?.starts_with(&path){
        return Err(ObtError::new(StatusCode::FORBIDDEN, "Try to escape from work directory"));
    }

    let f: File = File::open(path)
        .await?;
    let (read, buffer) = f.read_to_end_at(Vec::with_capacity(2048), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    match BASE64_STANDARD.decode(&buffer) {
        Ok(msgpack) => {
            Ok(web::HttpResponse::Ok()
                .header(header::CONTENT_TYPE, "application/msgpack")
                .body(msgpack))
        },
        Err(_) => {
            let message = String::from_utf8(buffer)?;
            Ok(web::HttpResponse::Ok()
                .header(header::CONTENT_TYPE, "application/json")
                .body(message))
        }
    }
}

async fn ok() -> Result<&'static str, ObtError> {
    Ok("")
}

#[ntex::main]
async fn main() -> Result<()> {
    let matches = command!()
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
            .default_value_if("no-tls", ArgPredicate::IsPresent, "127.0.0.1:80")
            .value_parser(value_parser!(SocketAddr)),
        )
        .arg(Arg::new("no-tls")
            .long("no-tls")
            .help("Disable TLS, will default to listen on port 80")
            .action(ArgAction::SetTrue)
        )
        .arg(Arg::new("work_dir").help("Specify where to find the respond files.").default_value(".").value_parser(value_parser!(PathBuf)))
        .get_matches();
    //let args: Vec<String> = std::env::args().collect();
    //OpenOptions::new().write(true).create(true).open("./args.txt").await?.write_all_at(args.join("\n"), 0).await.unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let use_tls = !matches.get_flag("no-tls");

    let server = || {
        set_current_dir(matches.get_one::<PathBuf>("work_dir").unwrap()).map_err(|e| anyhow!("Cannot change to work dir: {}", e))?;
        Ok::<_, anyhow::Error>(web::HttpServer::new(|| {
            web::App::new()
                .wrap(web::middleware::Logger::default())
                .service(web::resource("/200").to(ok))
                .service(web::resource("/ws").route(web::get().to(ws::ws_index)))
                .service(route::sync::hunter_sync)
                .default_service(web::route().to(default_handle))
        }))
    };
    
    let res: Result<()> = try { // Cannot break from try block
        if !use_tls {
            return Ok(server()?.bind(matches.get_one::<SocketAddr>("listen").unwrap())?.run().await?);
        }

        let cert_chain: Result<Vec<CertificateDer>> = try {
            ready::<Result<File>>(Ok(File::open(matches.get_one::<PathBuf>("crt").unwrap()).await?))
                .and_then(async |f: File| Ok(vec![CertificateDer::from_pem_slice(&f.read_to_end_at(Vec::with_capacity(4096), 0).await.unwrap().1)?])).await?
        };
        let cert_chain= cert_chain.map_err(|e: anyhow::Error| anyhow!("{} {}", "Failed to import certificate:", e))?;

        let key_der: Result<PrivateKeyDer> = try {
            ready::<Result<File>>(Ok(File::open(matches.get_one::<PathBuf>("key").unwrap()).await?))
            .and_then(async |f: File| Ok(PrivateKeyDer::from_pem_slice(&f.read_to_end_at(Vec::with_capacity(4096), 0).await.unwrap().1)?)).await?
        };
        let key_der = key_der.map_err(|e: anyhow::Error| anyhow!("{} {}", "Failed to import key:", e))?;

        let tls_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key_der)?;

        server()?.bind_rustls(matches.get_one::<SocketAddr>("listen").unwrap(), tls_config)?
            .run()
            .await?
    };
    res.map_err(|e: anyhow::Error| anyhow!("Failed to set up server:\n\t{}", e))
}