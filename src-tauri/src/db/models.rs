

use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use tokio::sync::Mutex;


#[derive( Debug, Clone)]
pub struct DbConnection(pub Arc<Mutex<Surreal<Db>>>);

impl DbConnection {
    pub fn get(&self) -> &Arc<Mutex<Surreal<Db>>> {
        &self.0
    }
}
