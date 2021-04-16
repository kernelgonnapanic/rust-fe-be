use crate::schema::products::dsl::*;
use diesel::{prelude::*, result::Error};

use crate::models;

pub fn get_products(db: &PgConnection) -> Result<Vec<models::Product>, Error> {
    products.get_results::<models::Product>(db)
}

pub fn insert_new_product(
    nm: &str,
    pr: i32,
    desc: &str,
    db: &PgConnection,
) -> Result<models::Product, Error> {
    let new_product = models::NewProduct {
        name: nm.to_owned(),
        price: pr,
        description: desc.to_owned(),
    };

    diesel::insert_into(products)
        .values(&new_product)
        .get_result(db)
}
