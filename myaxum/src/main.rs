use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(addr, app.into_make_service()).await.unwrap();
}

async fn root() -> &'static str {
    "Hello World!"
}

struct Test {
    hello: isize,
    world: String,
}
