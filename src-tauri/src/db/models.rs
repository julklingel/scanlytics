use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;


#[derive(Debug, Serialize)]
pub struct Name {
    pub first: String,
    pub last: String,
}

#[derive(Debug, Serialize)]
pub struct Person {
    pub title: String,
    pub name: Name,
    pub marketing: bool,
}


#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}
