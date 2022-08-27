extern crate dotenv;
#[macro_use] extern crate rocket;

use dotenv::dotenv;
use entities::{prelude::*, *};
use rocket::{serde::json::Json, State};
use sea_orm::{Database, DbErr, DatabaseConnection, EntityTrait};
use std::env;

mod entities;

#[get("/list")]
async fn list(db: &State<DatabaseConnection>) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;

    let games = Game::find()
        .into_json()
        .all(db)
        .await;
    games
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let db = Database::connect(db_url).await.unwrap();

    rocket::build()
        .manage(db)
        .mount("/", routes![list])
        .mount("/games", rocket::fs::FileServer::from("games/"))
}
