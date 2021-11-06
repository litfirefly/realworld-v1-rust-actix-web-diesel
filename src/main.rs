#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
mod app;
mod constants;
mod error;
mod middleware;
mod routes;
mod schema;
mod utils;

pub struct AppState {
    pub pool: utils::db::DbPool,
}

use crate::error::AppError;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
type AppPool = PooledConnection<ConnectionManager<PgConnection>>;
impl AppState {
    pub fn get_conn(&self) -> Result<AppPool, AppError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        let pool = utils::db::establish_connection();
        App::new()
            .wrap(logger)
            .data(AppState { pool })
            .wrap(middleware::cors::cors())
            .wrap(middleware::auth::Authentication)
            .service(web::scope("").configure(routes::api)) // TODO: call configure without emptpy scope
    })
    .bind(constants::BIND)?
    .run()
    .await
}
