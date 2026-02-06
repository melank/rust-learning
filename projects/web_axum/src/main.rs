use axum::Router;
use axum::extract::Path;
use axum::response::Json;
use axum::routing::get;
use std::collections::BTreeMap;

async fn health() -> &'static str {
    "ok"
}

async fn hello(Path(name): Path<String>) -> Json<BTreeMap<&'static str, String>> {
    let mut payload = BTreeMap::new();
    payload.insert("message", format!("Hello, {name}!"));
    Json(payload)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/hello/{name}", get(hello));

    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("8081"));
    let address = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("web_axum listening on http://{address}");
    axum::serve(listener, app).await?;

    Ok(())
}
