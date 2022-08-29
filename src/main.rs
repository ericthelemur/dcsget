extern crate dotenv;
#[macro_use] extern crate rocket;

use dotenv::dotenv;
use rocket::State;
use rocket::fs::TempFile;
use rocket::serde::{Serialize, Deserialize};
use rocket::form::Form;
use entities::{*, prelude::*};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use rocket::serde::json::Json;
use std::env;

mod entities;

#[get("/list")]
async fn list(db: &State<DatabaseConnection>) -> Json<Vec<game::Model>> {
    let db = db as &DatabaseConnection;
    Json(Game::find().all(db).await.unwrap())
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Commands {
    install: String,
    run: String,
}

#[get("/cmds/<id>")]
async fn commands(db: &State<DatabaseConnection>, id: u32) -> Option<Json<Commands>> {
    // TODO: Move to config and check these commands are actually correct lol
    let db = db as &DatabaseConnection;
    let install_dir = "~/gamingget";
    let host = "localhost:8000";
    if let Ok(Some(game)) = Game::find_by_id(id as i32).one(db).await {
        let archive = &game.archive;
        let url = host.to_owned() + "/games/" + archive;
        let ext = ".tar.gz";
        let output = if archive.ends_with(ext) { &archive[..archive.len() - ext.len()] } else { archive };
        return Some(Json(Commands {
            install: format!("cd \"{install_dir}\"; curl \"{url}\"; tar -xvf \"{archive}\" -o \"{output}\"; \"./{output}/setup.sh\""),
            run: format!("cd \"{install_dir}\"; \"./{output}/run.sh\""),
        }));
    } else {
        return None;
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let db = Database::connect(db_url).await.unwrap();

    rocket::build()
        .manage(db)
        .mount("/", routes![list, commands])
        .mount("/games", rocket::fs::FileServer::from("games/"))
}
