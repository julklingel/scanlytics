

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;



#[derive(Debug, Serialize, Deserialize)]
pub struct Testdata {
    pub id: Option<String>, // Optional if you want SurrealDB to generate IDs
    pub txt1: String,
    pub bool1: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: Thing,
    pub txt1: String,
    pub bool1: bool,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestdbResponse {
    pub id: String,
    pub bool1: bool,
    pub txt1: String,
}