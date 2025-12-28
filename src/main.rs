use axum::{
    Router,
    routing::{get, post},
};
use igreja;
#[tokio::main]
async fn main() {
    let pool = sqlx::SqlitePool::connect("sqlite:app.db").await.unwrap();
    let app = Router::new()
        .route("/", get(igreja::index))
        .route("/add_member", post(igreja::add_member))
        .with_state(pool);
    let addr = "0.0.0.0:3000";
    let adress = addr.strip_prefix("0.0.0.0:").unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("server is listening in address : http://127.0.0.1:{adress}/");
    axum::serve(listener, app).await.unwrap();
}
