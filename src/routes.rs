use crate::articles::*;
use actix_web::{get, web, HttpResponse, Responder};
use tracing::{error, info};

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
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_health() {
        //arrange
        let mut app = test::init_service(App::new().service(health)).await;
        let req = test::TestRequest::get().uri("/health").to_request();

        //act
        let resp = test::call_service(&mut app, req).await;

        //assert
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_home() {
        //arrange
        let mut app = test::init_service(App::new().service(home)).await;
        let req = test::TestRequest::get().uri("/").to_request();

        //act
        let resp = test::call_service(&mut app, req).await;

        //assert
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_about() {
        //arrange
        let mut app = test::init_service(App::new().service(about)).await;
        let req = test::TestRequest::get().uri("/about").to_request();

        //act
        let resp = test::call_service(&mut app, req).await;

        //assert
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_contact() {
        //arrange
        let mut app = test::init_service(App::new().service(contact)).await;
        let req = test::TestRequest::get().uri("/contact").to_request();

        //act
        let resp = test::call_service(&mut app, req).await;

        //assert
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_article() {
        //arrange
        let mut app = test::init_service(App::new().service(article)).await;
        let req = test::TestRequest::get().uri("/articles/20210715-Only-the-beginning.md").to_request();

        //act
        let resp = test::call_service(&mut app, req).await;

        //assert
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_article_fail() {
        //arrange
        let mut app = test::init_service(App::new().service(article)).await;
        let req = test::TestRequest::get().uri("/articles/test.md").to_request();

        //act
        let resp = test::call_service(&mut app, req).await;

        //assert
        assert_eq!(resp.status(), 404);
    }


}
