use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use futures::{sink::SinkExt, stream::StreamExt}; // ต้องใช้สองตัวนี้สำหรับ split และ next

// โครงสร้างข้อมูลสำหรับเก็บสถานะของ Server
struct AppState {
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    // 1. สร้าง Channel สำหรับกระจายเสียง (Broadcast)
    let (tx, _rx) = broadcast::channel::<String>(16);
    let app_state = Arc::new(AppState { tx });

    // 2. สร้าง Router
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Chat Server รันที่พอร์ต 3000");
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    // ระบุประเภทข้อมูลให้ axum::serve เพื่อแก้ error E0282
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    // แยกสายส่ง (sender) และสายรับ (receiver) ออกจากกัน
    let (mut sender, mut receiver) = socket.split();
    
    let mut rx = state.tx.subscribe();
    let tx = state.tx.clone();

    // Task ที่ 1: รับข้อความจาก Broadcast ของคนอื่น -> ส่งไปที่หน้าจอเรา
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Task ที่ 2: รับข้อความจากเรา -> ส่งไปที่ Broadcast ให้คนอื่นเห็น
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                let _ = tx.send(format!("User: {}", text));
            }
        }
    });

    // ถ้าใครคนใดคนหนึ่งตัดการเชื่อมต่อ ให้หยุดการทำงานของทั้งคู่
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}