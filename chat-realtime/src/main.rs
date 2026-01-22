use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
    Router,
};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    // สร้าง channel สำหรับส่งข้อความให้ทุกคน
    let (tx, _rx) = broadcast::channel::<String>(100);

    let app = Router::new()
        .route("/ws", get(move |ws: WebSocketUpgrade| {
            let tx = tx.clone();
            async move { ws.on_upgrade(|socket| handle_socket(socket, tx)) }
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_socket(mut socket: WebSocket, tx: broadcast::Sender<String>) {
    // โค้ดสำหรับจัดการรับ-ส่งข้อความจะอยู่ตรงนี้
}