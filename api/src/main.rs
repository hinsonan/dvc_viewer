#[macro_use] extern crate rocket;
extern crate serde_json;
use rocket::serde::{Deserialize, Serialize};
use rocket::response::stream::{EventStream, Event};
use rocket::serde::json::Json;
use rocket::form::Form;
use rocket::tokio::time::{self, Duration};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::fs;

#[derive(Serialize)]
struct FileListing{
    files: Vec<String>
}

#[derive(rocket::FromForm)]
struct FormData{
    name: String,
    age: i8
}

fn display_files(dir_path: &str) -> Vec<String>{
    let mut string_list: Vec<String> = Vec::new();
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        string_list.push(file_name);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
        }
    }
    return string_list;
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/send_data", data = "<payload>")]
fn send_data(payload: Form<FormData>) -> String {
    // Access the payload data
    let field1 = &payload.name;
    let field2 = payload.age;

    // Process the data
    // ...

    // Return a response
    format!("Received data: {} - {}", field1, field2)
}

#[get("/files")]
fn files() -> Json<FileListing>{
    let x: Vec<String> = display_files(".");
    let obj = FileListing{files:x};
    Json(obj)
}

#[get("/events")]
fn stream() -> EventStream![] {
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            yield Event::data("ping");
            interval.tick().await;
        }
    }
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);
    rocket::build().attach(cors.to_cors().unwrap()).mount("/", routes![index,files,send_data,stream])
}
