use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::connect_async;
use url::Url;
use futures_util::StreamExt;
use futures_util::SinkExt;

async fn connect_websocket(url: &Url, duration: Duration, count: u32) {
    let (ws_stream, _) = connect_async(url.clone()).await.expect("Failed to connect");
    let (mut write, _) = ws_stream.split();
    // Keep the connection open for the specified duration
    write.send(tokio_tungstenite::tungstenite::Message::Text(count.to_string()))
        .await
        .expect("Failed to send message");

    // Close the connection
    // write.close().await.expect("Failed to close connection");
}

#[tokio::main]
async fn main() {
    // Set the WebSocket URL
    let url = Url::parse("ws://localhost:3000/ws").expect("Invalid WebSocket URL");

    // Set the duration to keep each connection open
    let duration = Duration::from_secs(10);

    // Set the number of connections to make
    let connection_count = 1_000_000;

    // Connect to the WebSocket URL the specified number of times
    for i in 0..connection_count {
        connect_websocket(&url, duration, i).await;
    }
}


