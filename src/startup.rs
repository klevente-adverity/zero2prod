use std::net::TcpListener;

use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;

use crate::routes;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(connection.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}
