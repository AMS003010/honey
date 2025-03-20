use actix_web::{web, App, HttpServer};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

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
    // Initialize DB connection
    let db: Surreal<Client> = Surreal::init();
    
    // Connect to SurrealDB
    db.connect::<Ws>("localhost:8000").await?;

    // Sign in to SurrealDB
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    // Use namespace and database
    db.use_ns("namespace").use_db("database").await?;
    
    println!("\n✅ Connected to SurrealDB!");
    
    // Share the DB connection with handlers
    let db_data = web::Data::new(db);
    
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(web::scope("/api")
                .service(routes::todo_routes::create_todo)
                .service(routes::todo_routes::get_todo)
                .service(routes::todo_routes::update_todo)
                .service(routes::todo_routes::delete_todo)
                .service(routes::todo_routes::get_all_todo)
            )
    })
    .bind(("localhost", 8080))?
    .workers(4)
    .shutdown_timeout(60)
    .run();
    
    println!("🚀 Server running at http://localhost:8080");
    server.await?;
    Ok(())
}