

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;



#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}


#[derive(Debug, Serialize)]
pub struct Testdata {
    pub txt1: String,
    pub bool1: bool
}
