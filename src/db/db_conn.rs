use crate::DB;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

pub async fn db_init() -> Result<(), Box<dyn std::error::Error>> {
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("namespace").use_db("database").await?;

    println!("✅ Connected to SurrealDB!");
    Ok(())
}