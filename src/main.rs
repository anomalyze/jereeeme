use crate::routes::*;
use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use tracing_subscriber;

pub mod articles;
pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    HttpServer::new(|| {
        App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("X-Frame-Options", "SAMEORIGIN")
                    .header("X-XSS-Protection", "1; mode=block")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("Content-Type", "text/html"),
            )
            .service(home)
            .service(about)
            .service(contact)
            .service(health)
            .service(article)
            .service(Files::new("/assets", "./assets"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
