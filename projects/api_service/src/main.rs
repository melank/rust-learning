mod app;
mod db;
mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::build_app().await?;

    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("8083"));
    let address = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("api_service listening on http://{address}");
    axum::serve(listener, app).await?;

    Ok(())
}
