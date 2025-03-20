use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use std::sync::Arc;

pub struct DbPool {
    connections: Vec<Arc<Surreal<Client>>>,
    current_index: std::sync::atomic::AtomicUsize,
}

impl DbPool {
    pub async fn new(size: usize, url: &str, username: &str, password: &str, namespace: &str, database: &str) -> Result<Self, surrealdb::Error> {
        let mut connections = Vec::with_capacity(size);
        
        for _ in 0..size {
            let db: Surreal<Client> = Surreal::init();
            db.connect::<Ws>(url).await?;
            db.signin(Root {
                username,
                password,
            }).await?;
            db.use_ns(namespace).use_db(database).await?;
            
            connections.push(Arc::new(db));
        }
        
        Ok(Self { 
            connections,
            current_index: std::sync::atomic::AtomicUsize::new(0),
        })
    }
    
    pub fn get_connection(&self) -> Arc<Surreal<Client>> {
        // Simple round-robin connection selection
        let index = self.current_index.fetch_add(1, std::sync::atomic::Ordering::SeqCst) % self.connections.len();
        self.connections[index].clone()
    }
}