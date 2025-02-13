use futures::future::ready;
use log::{info, warn};
use ntex::{chain, fn_service, service::{fn_factory_with_config, fn_shutdown}, web, Service};

use crate::error::ObtError;

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
pub(crate) async fn ws_index(req: web::HttpRequest) -> Result<web::HttpResponse, ObtError> {
    //println!("serve /ws");
    web::ws::start::<_, _, ObtError>(req, fn_factory_with_config(ws_service)).await
}
