use serde::{Deserialize, Serialize};

use crate::schema::products;
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub price: i32,
    pub description: String,
}
