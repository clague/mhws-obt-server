
use std::borrow::Cow;

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use ntex::{http::{header, StatusCode}, web::{self, DefaultError, FromRequest, HttpRequest}};

use crate::error::ObtError;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct SaveSlots<'a> {
    #[serde(borrow)]
    hunter_save_list: Vec<HunterSave<'a>>,
    using_save_slot: u32
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct SyncSaveSlots<'a> {
    invalid_save_slot_info_list: Option<Vec<HunterSave<'a>>>,
    invalid_client_hunter_id_list: Option<String>,
    save_slot_info_list: Vec<HunterSave<'a>>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct HunterSave<'a> {
    hunter_id: Cow<'a, str>, 
    hunter_name: Cow<'a, str>,
    otomo_name: Cow<'a, str>,
    save_slot: u32,
    short_id: Option<Cow<'a, str>>,
}

#[web::post("/hunter/sync")]
async fn hunter_sync<'a>(mut save_slots: SaveSlots<'a>) -> Result<impl web::Responder, ObtError> {
    let mut sync_save_slots = SyncSaveSlots {
        invalid_save_slot_info_list: None,
        invalid_client_hunter_id_list: None,
        save_slot_info_list: Vec::new()
    };
    save_slots.hunter_save_list.iter_mut().for_each(|s| s.short_id = Some(Cow::Borrowed("NX8684EC")));
    sync_save_slots.save_slot_info_list.append(&mut save_slots.hunter_save_list);
    let mut buf = Vec::new();
    sync_save_slots.serialize(&mut Serializer::new(&mut buf).with_struct_map()).unwrap();
    //println!("serve /MultiplayerServer/ListPartyQosServers?sdk={}", info.sdk);
    Ok(web::HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/msgpack")
        .body(buf))
}

const MAX_SIZE: usize = 262_144;

impl<'a> FromRequest<DefaultError> for SaveSlots<'a> {

    fn from_request(
        req: &HttpRequest,
        payload: &mut ntex::http::Payload,
    ) -> impl Future<Output = Result<Self, Self::Error>> {
        async {
            if req.headers().get(header::CONTENT_TYPE).is_some_and(|h| h != "application/msgpack") {
                return Err(ObtError::new(StatusCode::FORBIDDEN, "Request wrong content type"));
            }
            let mut body = serde_bytes::ByteBuf::new();
            while let Some(chunk) = payload.next().await {
                let chunk = chunk?;
                // limit max size of in-memory payload
                if (body.len() + chunk.len()) > MAX_SIZE {
                    return Err(ObtError::new(StatusCode::FORBIDDEN, "request body overflow"));
                }
                body.extend_from_slice(&chunk);
            }
            serde::de::Deserialize::deserialize(&mut Deserializer::new(body.as_slice())).map_err(|e| 
                ObtError::new(StatusCode::INTERNAL_SERVER_ERROR, &format!("Cannot deserialize message: {}", &e)))
        }
    }
    
    type Error = ObtError;
}