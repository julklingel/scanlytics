mod init;
mod models;

pub use init::{init_db, define_db_on_startup, setup_database};
pub use models::DbConnection;


pub use surrealdb::{
    Surreal,
    engine::any::Any,
    engine::local::Mem,
    sql::{Thing, Datetime}, 
    Error
    
};