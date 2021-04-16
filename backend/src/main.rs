#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

mod actions;
mod models;
mod schema;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/product")]
async fn add_product(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewProduct>,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let product = web::block(move || {
        actions::insert_new_product(&form.name, form.price, &form.description, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(product))
}

#[get("/products")]
async fn products(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let db = pool.get().expect("couldn't get db connection from pool");

    let products = web::block(move || actions::get_products(&db))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(products))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Compress::default())
            .service(add_product)
            .service(products)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
