use std::random::random;

use compio::{fs::File, io::AsyncReadAtExt};
use ntex::{http::header, web};
use crate::error::ObtError;

#[web::post("/v1/projects/earth-analysis-obt/topics/analysis-client-log:publish")]
async fn earth_analysis_obt() -> Result<impl web::Responder, ObtError> {
    let zh_hans_json: File = File::open("earth-analysis-obt.json").await?;
    let (read, buffer) = zh_hans_json.read_to_end_at(Vec::with_capacity(1024), 0).await.unwrap();
    assert_eq!(read, buffer.len());
    //println!("serve /v1/projects/earth-analysis-obt/topics/analysis-client-log:publish");
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(String::from_utf8(buffer)?.replace("{random_number}", &random::<u32>().to_string())))
}