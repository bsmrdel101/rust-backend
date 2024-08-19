use tide_websockets::{Message, WebSocket, WebSocketConnection};
use std::sync::Arc;
use futures_lite::stream::StreamExt;
use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize)]
struct WebSocketMessage {
    event: String,
    params: Value,
}

async fn websocket_handler(
    _req: tide::Request<Arc<sqlx::PgPool>>,
    mut stream: WebSocketConnection
) -> tide::Result<()> {
    while let Some(Ok(Message::Text(text))) = stream.next().await {
        let parsed: Result<WebSocketMessage, _> = serde_json::from_str(&text);
        match parsed {
            Ok(message) => {
                println!("Event: {}", message.event);
                println!("Params: {:?}", message.params);

                match message.event.as_str() {
                    "JOIN_ROOM" => {

                    }
                    _ => { eprintln!("No server event found."); }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse message: {}", e);
            }
        }
    }
    Ok(())
}

pub fn register(app: &mut tide::Server<Arc<sqlx::PgPool>>) {
    app.at("/ws").get(WebSocket::new(websocket_handler));
}
