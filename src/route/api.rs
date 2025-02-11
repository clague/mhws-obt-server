use compio::{fs::File, io::AsyncReadAtExt};
use ntex::{http::header, web};
use crate::error::ObtError;
use base64::prelude::*;

#[web::post("/auth/login")]
async fn auth_login() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("auth-login.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /auth/login");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/delivery_data/get")]
async fn delivery_data() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("delivery-data.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /delivery_data/get");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/hunter/sync")]
async fn hunter_sync() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("hunter-sync.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /hunter/sync");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/hunter/delete")]
async fn hunter_delete() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("hunter-delete.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /hunter/delete");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/hunter/profile/update")]
async fn hunter_profile_update() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("hunter-profile-update.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /hunter/profile/update");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/obt/play")]
async fn obt_play() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("obt-play.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /obt/play");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/hunter/character_creation/upload")]
async fn hunter_character_creation_upload() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("hunter-character-creation-upload.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /hunter/character_creation/upload");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/auth/ticket")]
async fn auth_ticket() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("auth-ticket.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /auth/ticket");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/follow/total_list")]
async fn follow_total_list() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("follow-total_list.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /follow/total_list");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/offline/notification_list")]
async fn offline_notification_list() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("offline-notification_list.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /offline/notification_list");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/community/invitation/received_list")]
async fn community_invitation_received_list() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("community-invitation-received_list.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /community/invitation/received_list");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/block/list")]
async fn block_list() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("block-list.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /block/list");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}

#[web::post("/friend/list")]
async fn friend_list() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("friend-list.base64").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /friend/list");
    let msgpack = BASE64_STANDARD.decode(buffer)?;
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(msgpack))
}