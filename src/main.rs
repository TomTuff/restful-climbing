use actix_web::{HttpServer, App, Responder, HttpResponse, post};

pub mod route;
mod routes;

// macro for generating the app so that we don't have redundant code in tests module and main()
#[macro_export]
macro_rules! app (
    () => ({
        App::new()
            .service(routes::routes_post)
    });
);


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let x = actix_web::App::new();
    HttpServer::new(|| {
        app!()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


#[cfg(test)]
mod tests {
    use super::*;
    use route::Route;
    use actix_web::{
        http::{self},
        test,
    };

    fn test_route() -> Route {
        Route::new("funky monkey".to_string(), 5, (123.45, 52.310))
    }

    #[actix_web::test]
    async fn test_adding_route() {
        let app = test::init_service(app!()).await;
        let req = test::TestRequest::post().uri("/routes").set_json(test_route()).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK)
    }

    #[actix_web::test]
    async fn test_adding_route_fails() {
        let app = test::init_service(app!()).await;
        let req = test::TestRequest::post().uri("/routes").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST)
    }
}