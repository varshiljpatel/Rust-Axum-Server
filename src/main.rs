use axum::{ routing::get, Router };
mod mymod;

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", get(root));
    let listner: tokio::net::TcpListener = tokio::net::TcpListener
        ::bind("0.0.0.0:8000").await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}

async fn root() -> &'static str {
    "hello varshil"
}
