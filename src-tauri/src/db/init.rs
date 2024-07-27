use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tokio::sync::RwLock;

pub async fn init_db() -> Result<RwLock<Surreal<Client>>, String> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.map_err(|e| e.to_string())?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .map_err(|e| e.to_string())?;
    db.use_ns("namespace").use_db("database").await.map_err(|e| e.to_string())?;
    Ok(RwLock::new(db))
}