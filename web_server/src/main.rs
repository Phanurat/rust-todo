use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // 1. สร้าง Route (เส้นทางของเว็บ)
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust Server! 🦀" }))
        .route("/ping", get(handler_ping));

    // 2. กำหนด Address (Localhost พอร์ต 3000)
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server กำลังรันที่ http://localhost:3000");

    // 3. เริ่มต้นรัน Server
    axum::serve(listener, app).await.unwrap();
}

async fn handler_ping() -> &'static str {
    "Pong! 🏓"
}