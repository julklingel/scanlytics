use std::sync::Arc;
use surrealdb::Surreal;


use tokio::sync::Mutex;
use surrealdb::engine::any::Any;

#[derive(Debug, Clone)]
pub struct DbConnection(pub Arc<Mutex<Surreal<Any>>>);

impl DbConnection {
    pub fn get(&self) -> &Arc<Mutex<Surreal<Any>>> {
        &self.0
    }
}