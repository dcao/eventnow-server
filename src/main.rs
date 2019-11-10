use actix_web::{get, middleware::Logger, post, delete, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::Deserialize;

#[macro_use]
extern crate diesel;

pub mod db;
pub mod models;
pub mod schema;

#[get("/groups")]
fn get_groups(pool: web::Data<db::Pool>) -> impl Responder {
    match db::get_all_groups(&pool.get().unwrap()) {
        Ok(gs) => HttpResponse::Ok().json(gs),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/groups")]
fn create_group(pool: web::Data<db::Pool>, json: web::Json<models::NewGroup>) -> impl Responder {
    match db::create_group(&pool.get().unwrap(), json.0) {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/groups/{group_id}")]
fn get_group_by_id(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    match db::get_group_by_id(&pool.get().unwrap(), *id) {
        Ok(g) => HttpResponse::Ok().json(g),
        Err(db::DbErr::NotFound) => HttpResponse::NoContent().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[delete("/groups/{group_id}")]
fn delete_group(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    match db::delete_group(&pool.get().unwrap(), *id) {
        Ok(_) => HttpResponse::Ok(),
        Err(db::DbErr::NotFound) => HttpResponse::NoContent().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/groups/{group_id}/members")]
fn get_users_in_group(pool: web::Data<db::Pool>, group_id: web::Path<i32>) -> impl Responder {
    match db::get_users_in_group(&pool.get().unwrap(), *group_id) {
        Ok(us) => HttpResponse::Ok().json(us),
        Err(db::DbErr::NotFound) => HttpResponse::NoContent().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
    

#[derive(Deserialize)]
struct AddGroupMember {
    pub user_id: i32,
    pub role: models::GroupRole,
}

#[post("/groups/{group_id}/members")]
fn add_group_member(
    pool: web::Data<db::Pool>,
    json: web::Json<AddGroupMember>,
    group_id: web::Path<i32>,
) -> impl Responder {
    let nmship = models::NewMembership {
        group_id: *group_id,
        user_id: json.0.user_id,
        membership_role: json.0.role,
    };

    match db::create_membership(&pool.get().unwrap(), nmship) {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/events")]
fn get_events(pool: web::Data<db::Pool>) -> impl Responder {
    match db::get_all_events(&pool.get().unwrap()) {
        Ok(gs) => HttpResponse::Ok().json(gs),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/events")]
fn create_event(pool: web::Data<db::Pool>, json: web::Json<models::NewEvent>) -> impl Responder {
    match db::create_event(&pool.get().unwrap(), json.0) {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/events/{event_id}")]
fn get_event_by_id(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    match db::get_event_by_id(&pool.get().unwrap(), *id) {
        Ok(g) => HttpResponse::Ok().json(g),
        Err(db::DbErr::NotFound) => HttpResponse::NoContent().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[delete("/events/{event_id}")]
fn delete_event(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    match db::delete_event(&pool.get().unwrap(), *id) {
        Ok(_) => HttpResponse::Ok(),
        Err(db::DbErr::NotFound) => HttpResponse::NoContent().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/users")]
fn get_users(pool: web::Data<db::Pool>) -> impl Responder {
    match db::get_all_users(&pool.get().unwrap()) {
        Ok(gs) => HttpResponse::Ok().json(gs),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/users")]
fn create_user(pool: web::Data<db::Pool>, json: web::Json<models::NewUser>) -> impl Responder {
    match db::create_user(&pool.get().unwrap(), json.0) {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/users/{user_id}")]
fn get_user_by_id(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    match db::get_user_by_id(&pool.get().unwrap(), *id) {
        Ok(g) => HttpResponse::Ok().json(g),
        Err(db::DbErr::NotFound) => HttpResponse::NoContent().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[delete("/users/{user_id}")]
fn delete_user(pool: web::Data<db::Pool>, id: web::Path<i32>) -> impl Responder {
    match db::delete_user(&pool.get().unwrap(), *id) {
        Ok(_) => HttpResponse::Ok(),
        Err(db::DbErr::NotFound) => HttpResponse::NoContent().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/users/{user_id}/groups")]
fn get_users_groups(pool: web::Data<db::Pool>, user_id: web::Path<i32>) -> impl Responder {
    match db::get_groups_of_user(&pool.get().unwrap(), *user_id) {
        Ok(gs) => HttpResponse::Ok().json(gs),
        Err(db::DbErr::NotFound) => HttpResponse::NoContent().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
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
            .service(create_group)
            .service(get_events)
            .service(create_event)
            .service(get_users)
            .service(create_user)
            .service(delete_event)
            .service(delete_group)
            .service(delete_user)
    })
    .bind("127.0.0.1:5000")
    .unwrap()
    .run()
    .unwrap();
}
