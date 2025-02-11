use compio::{fs::File, io::AsyncReadAtExt};
use ntex::{http::header, web};
use crate::error::ObtError;

#[web::post("/v1/steam-steam/sign/EAR-B-WW")]
async fn steam_sign() -> Result<impl web::Responder, ObtError> {
    let system_json: File = File::open("EAR-B-WW.json").await?;
    let (read, buffer) = system_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /v1/steam-steam/sign/EAR-B-WW");
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(String::from_utf8(buffer)?))
}

#[web::get("/v1/consent/restrictions/{country}")]
async fn consent_restrictions(path: web::types::Path<String>) -> Result<impl web::Responder, ObtError> {
    let country = path.into_inner();
    let system_json: File = File::open("consent-restrictions.json").await?;
    let (read, buffer) = system_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /v1/consent/restrictions/{country}");
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(String::from_utf8(buffer)?.replace("{country}", &country)))
}

#[web::get("/v1/consent/countries/{language}")]
async fn consent_countries(path: web::types::Path<String>) -> Result<impl web::Responder, ObtError> {
    let language = path.into_inner();
    let system_json: File = File::open("consent-countries.json").await?;
    let (read, buffer) = system_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /v1/consent/countries/{language}");
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(String::from_utf8(buffer)?.replace("{language}", &language)))
}

#[web::get("/v1/consent/documents/EAR-B-WW/{country}/{language}/over")]
async fn consent_documents(path: web::types::Path<(String, String)>) -> Result<impl web::Responder, ObtError> {
    let (country, language) = path.into_inner();
    let system_json: File = File::open("consent-documents.json").await?;
    let (read, buffer) = system_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /v1/consent/documents/EAR-B-WW/{country}/{language}/over");
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(String::from_utf8(buffer)?.replace("{country}", &country).replace("{language}", &language)))
}

#[web::get("/v1/token/refresh/")]
async fn token_refresh() -> Result<impl web::Responder, ObtError> {
    let system_json: File = File::open("token-refresh.json").await?;
    let (read, buffer) = system_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /v1/token/refresh");
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(String::from_utf8(buffer)?))
}