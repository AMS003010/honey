use actix_web::{web, App, HttpServer};
use db::db_pool::DbPool;

mod db {
    pub mod db_pool;
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

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a connection pool with 5 connections
    let pool = DbPool::new(
        5, 
        "localhost:8000", 
        "root", 
        "root", 
        "namespace", 
        "database"
    ).await?;
    
    println!("\n✅ Connected to SurrealDB with pool of 5 connections!");
    
    // Share the pool with handlers
    let pool_data = web::Data::new(pool);
    
    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .service(routes::todo_routes::create_todo)
            .service(routes::todo_routes::get_todo)
            .service(routes::todo_routes::update_todo)
            .service(routes::todo_routes::delete_todo)
            .service(routes::todo_routes::get_all_todo)
    })
    .bind(("localhost", 8080))?
    .workers(4)
    .shutdown_timeout(60)
    .run();
    
    println!("🚀 Server running at http://localhost:8080");
    server.await?;
    Ok(())
}