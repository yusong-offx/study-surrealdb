use std::future::IntoFuture;

use axum::routing::get;
use axum::Router;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let addr = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();

    db.use_ns("test").use_db("test").await.unwrap();

    let create_result: Vec<User> = db
        .create("Users")
        .content(User {
            user_id: String::from("user_id"),
            password: String::from("password"),
            user_name: String::from("user_name"),
        })
        .await
        .unwrap();
    dbg!(create_result);

    let read_result: Vec<ReadUser> = db.select("Users").await.unwrap();
    dbg!(read_result);

    axum::serve(addr, app.into_make_service()).await.unwrap();
}

async fn root() -> &'static str {
    "Hello World!"
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    user_id: String,
    password: String,
    user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReadUser {
    id: Thing,
    user_id: String,
    password: String,
    user_name: String,
}

struct Test {
    hello: isize,
    world: String,
}
