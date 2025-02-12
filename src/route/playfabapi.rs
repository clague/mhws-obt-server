use compio::{fs::File, io::AsyncReadAtExt};
use ntex::{http::header, web};
use crate::error::ObtError;


#[web::post("/MultiplayerServer/ListPartyQosServers")]
async fn list_servers(_info: web::types::Query<String>) -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("PartyQosServers.json").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /MultiplayerServer/ListPartyQosServers?sdk={}", info.sdk);
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(String::from_utf8(buffer)?))
}