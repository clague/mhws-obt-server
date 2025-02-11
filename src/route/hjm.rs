use compio::{fs::File, io::AsyncReadAtExt};
use ntex::{http::header, web};
use crate::error::ObtError;

#[web::get("/systems/EAR-B-WW/00002/system.json")]
async fn system_json() -> Result<impl web::Responder, ObtError> {
    let system_json: File = File::open("system.json").await?;
    let (read, buffer) = system_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /systems/EAR-B-WW/00002/system.json");
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(String::from_utf8(buffer)?))
}

#[web::get("/consents/EAR-B-WW/analysis/1/{language}.json")]
async fn analysis(path: web::types::Path<String>) -> Result<impl web::Responder, ObtError> {
    let language = path.into_inner();
    let zh_hans_json: File = File::open("analysis-languages.json").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /consents/EAR-B-WW/analysis/1/{language}.json");
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(String::from_utf8(buffer)?.replace("{language}", &language)))
}
