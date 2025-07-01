use axum::{Router, debug_handler, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    const ADDR: &str = "0.0.0.0:3000";

    // 创建路由
    let app = Router::new()
        .route("/", routing::get(index))
        .route("/health", routing::get(health_check));

    // 绑定地址和端口
    let listener = TcpListener::bind(ADDR).await.unwrap();
    println!("🚀 服务器启动成功！");
    println!("📍 地址: http://{}", ADDR);

    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello Rust!"
}

#[debug_handler]
async fn health_check() -> &'static str {
    "OK"
}
