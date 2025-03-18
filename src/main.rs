use actix_web::{App, HttpServer};
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use routes::todo_routes::{createTodo, getTodo, updateTodo, deleteTodo, getAllTodo};
use db::db_conn::db_init;

mod db {
    pub mod db_conn;
}

mod models {
    pub mod todo;
}

mod routes {
    pub mod todo_routes;
}

mod utils {
    pub mod error;
}

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    db_init().await?;
    
    let server = HttpServer::new(|| {
        App::new()
            .service(createTodo)
            .service(getTodo)
            .service(updateTodo)
            .service(deleteTodo)
            .service(getAllTodo)
    })
    .bind(("localhost",8080))?
    .workers(4)
    .shutdown_timeout(60)
    .run();

    println!("🚀 Server running at http://localhost:8080");
    server.await.unwrap();
    Ok(())
}