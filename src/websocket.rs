use actix_web::{web, HttpRequest, Responder};
use actix_ws::Message;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::models::MinaVersion;

#[derive(Deserialize, Serialize)]
pub struct WebsocketMessage {
    pub(crate) channel: String,
    pub(crate) data: String,
}

fn create_response<T: Serialize>(channel: &str, data: T) -> String {
    let response = WebsocketMessage {
        channel: channel.to_string(),
        data: serde_json::to_string(&data).unwrap(),
    };

    serde_json::to_string(&response).unwrap()
}

pub async fn ws(req: HttpRequest, body: web::Payload, app_state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(msg) => {
                    let WebsocketMessage {channel, data} = serde_json::from_str(&msg).unwrap();

                    match channel.as_str() {
                        "version" => {
                            let mina_version = MinaVersion::get(&app_state.config.mina.graphql_url).await;
                            let _ = session.text(create_response(&channel, mina_version)).await;
                        }
                        _ => {
                            let _ = session.text(create_response(&channel, "Invalid channel")).await;
                        }
                    }
                },
                _ => break,
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}