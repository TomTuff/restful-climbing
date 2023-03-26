use actix_web::{middleware::Logger, App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;

pub mod error;
pub mod pg;
pub mod route;
mod routes;

// macro for generating the app so that we don't have redundant code in tests module and main()
#[macro_export]
macro_rules! app (
    () => ({
        dotenv().ok();
        let _ = env_logger::try_init_from_env(Env::default().default_filter_or("info"));  // assign to _ because Result<(), SetLoggerError> is intentionally unused; SetLoggerError indicates set_logger was already called, which is fine.
        App::new()
            .wrap(Logger::default())
            .service(routes::routes_post)
            .service(routes::routes_get)
    });
);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| app!())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self},
        test,
    };
    use route::{DifficultyRating, Route};

    fn test_route() -> Route {
        Route::new(
            "funky monkey".to_string(),
            DifficultyRating::Rating59,
            123.45,
            52.310,
        )
    }

    #[actix_web::test]
    async fn test_adding_route() {
        let app = test::init_service(app!()).await;
        let req = test::TestRequest::post()
            .uri("/routes")
            .set_json(test_route())
            .to_request();
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

    // #[actix_web::test]
    // async fn populate_some_routes() {
    //     let app = test::init_service(app!()).await;
    //     let fake_names = vec!["alluring alligator", "third bird", "fat cat", "flyin' lion", "swish fish", "free tree", "power flower", "sheet meat", "red lead"];
    //     for fake_name in fake_names.iter() {
    //         let mut route = test_route();
    //         route.name = fake_name.to_string();
    //         let req = test::TestRequest::post()
    //             .uri("/routes")
    //             .set_json(route)
    //             .to_request();
    //         let resp = test::call_service(&app, req).await;
    //         assert_eq!(resp.status(), http::StatusCode::OK);
    //     }
    // }

    #[actix_web::test]
    async fn test_get_routes() {
        let app = test::init_service(app!()).await;
        let req = test::TestRequest::get().uri("/routes").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body: Vec<Route> = test::read_body_json(resp).await;
        println!("GET /routes response:\n{:?}", body);
    }
}
