use actix_web::{get, web, HttpResponse, Responder};
use tracing::{info, error};
use crate::articles::*;

#[get("/articles/{id}")]
pub async fn article(article: web::Path<String>) -> impl Responder {
    info!("article | request: {}", article);
    Article::build(&format!("./articles/{}", article))?.await
}

#[get("/")]
pub async fn home() -> impl Responder {
    info!("home    | request");
    match Blog::Home.build() {
        Ok(page) => {
            info!("home    | response: found");
            HttpResponse::Ok().body(page)
        },
        Err(_) => {
            error!("home    | response: error");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/about")]
pub async fn about() -> impl Responder {
    info!("about   | request");
    match Blog::About.build() {
        Ok(page) => {
            info!("about   | response: found");
            HttpResponse::Ok().body(page)
        }
        Err(_) => {
            error!("about   | response: error");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/contact")]
pub async fn contact() -> impl Responder {
    info!("contact | request");
    match Blog::Contact.build() {
        Ok(page) => {
            info!("contact | response: found");
            HttpResponse::Ok().body(page)
        }
        Err(_) => {
            error!("contact  | response: error");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/health")]
pub async fn health() -> impl Responder {
    info!("health  | request");
    HttpResponse::Ok()
}
