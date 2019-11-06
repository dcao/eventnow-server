use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

#[macro_use]
extern crate diesel;

pub mod db;
pub mod models;
pub mod schema;

#[get("/groups")]
fn get_groups(pool: web::Data<db::Pool>) -> impl Responder {
    match db::groups(&pool.get().unwrap()) {
        Ok(gs) => HttpResponse::Ok().json(gs),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[derive(Deserialize)]
struct AddGroupData {
    name: String,
    description: String,
}

#[post("/groups")]
fn add_group(json: web::Json<AddGroupData>, pool: web::Data<db::Pool>) -> impl Responder {
    let new_group = models::NewGroup {
        name: &json.name,
        description: Some(&json.description),
        cover_img: None,
        profile_img: None,
    };

    match db::create_group(&pool.get().unwrap(), new_group) {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/events")]
fn get_events() -> impl Responder {
    HttpResponse::Ok().body("Getting events")
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .service(get_groups)
            .service(add_group)
            .service(get_events)
    })
    .bind("127.0.0.1:5000")
    .unwrap()
    .run()
    .unwrap();
}
