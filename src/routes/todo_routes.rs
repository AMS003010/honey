use crate::utils::error::Error;
use crate::models::todo::Todo;
use crate::DB;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use surrealdb::sql::Thing;

#[post("/todo/{id}")]
pub async fn createTodo(path: web::Path<String>, body: web::Json<Todo>) -> impl Responder {
    let id = path.into_inner();
    let todo = body.into_inner();

    let created: Result<Option<Todo>, _> = DB.create(("todo", id))
        .content(todo)
        .await;

    match created {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::InternalServerError().body("Failed to create Todo"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[get("/todo/{id}")]
pub async fn getTodo(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let fetched: Result<Option<Todo>, _> = DB.select(("todo", id)).await;

    match fetched {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[put("/todo/{id}")]
pub async fn updateTodo(path: web::Path<String>, body: web::Json<Todo>) -> impl Responder {
    let id = path.into_inner();
    let todo = body.into_inner();

    let updated: Result<Option<Todo>, _> = DB.update(("todo", id))
        .merge(todo)
        .await;

    match updated {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[delete("/todo/{id}")]
pub async fn deleteTodo(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let deleted: Result<Option<Todo>, _> = DB.delete(("todo", id)).await;

    match deleted {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[get("/todo")]
pub async fn getAllTodo() -> impl Responder {
    let result: Result<Vec<Todo>, _> = DB.select("todo").await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}
