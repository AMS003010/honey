use crate::utils::error::Error;
use crate::models::todo::Todo;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;

#[post("/todo/{id}")]
pub async fn create_todo(
    path: web::Path<String>, 
    body: web::Json<Todo>,
    db: web::Data<Surreal<Client>>
) -> impl Responder {
    let id = path.into_inner();
    let todo = body.into_inner();

    let created: Result<Option<Todo>, _> = db.create(("todo", id))
        .content(todo)
        .await;

    match created {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::InternalServerError().body("Failed to create Todo"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[get("/todo/{id}")]
pub async fn get_todo(
    path: web::Path<String>,
    db: web::Data<Surreal<Client>>
) -> impl Responder {
    let id = path.into_inner();

    let fetched: Result<Option<Todo>, _> = db.select(("todo", id)).await;

    match fetched {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[put("/todo/{id}")]
pub async fn update_todo(
    path: web::Path<String>, 
    body: web::Json<Todo>,
    db: web::Data<Surreal<Client>>
) -> impl Responder {
    let id = path.into_inner();
    let todo = body.into_inner();

    let updated: Result<Option<Todo>, _> = db.update(("todo", id))
        .merge(todo)
        .await;

    match updated {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[delete("/todo/{id}")]
pub async fn delete_todo(
    path: web::Path<String>,
    db: web::Data<Surreal<Client>>
) -> impl Responder {
    let id = path.into_inner();

    let deleted: Result<Option<Todo>, _> = db.delete(("todo", id)).await;

    match deleted {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[get("/todo")]
pub async fn get_all_todo(
    db: web::Data<Surreal<Client>>
) -> impl Responder {
    let result: Result<Vec<Todo>, _> = db.select("todo").await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}