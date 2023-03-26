use actix_web::{middleware::Logger, web, App, HttpServer};
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
            .service(
                web::scope("/routes")
                    .service(routes::add_new_route)
                    .service(routes::get_recent_routes)
                    .service(routes::get_route_by_id)
                    .service(routes::delete_route_by_id)
                    .service(routes::update_route_by_id)
            )
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
    use crate::route::NumberRoutes;

    use super::*;
    use actix_web::{
        http::{self},
        test,
    };
    use route::{DifficultyRating, Route};

    fn test_route() -> Route {
        Route::new(
            None,
            "funky monkey".to_string(),
            DifficultyRating::Rating59,
            123.45,
            52.310,
        )
    }

    #[actix_web::test]
    async fn test_add_get_delete_route() {
        let app = test::init_service(app!()).await;

        // Add route
        let req = test::TestRequest::post()
            .uri("/routes")
            .set_json(test_route())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Get route
        let one_route = NumberRoutes { number_routes: 1 };
        let req = test::TestRequest::get()
            .uri("/routes")
            .set_json(one_route)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body: Vec<Route> = test::read_body_json(resp).await;
        assert_eq!(body.len(), 1);
        let id = body[0].id.unwrap();
        println!("returned id for funky monkey: {id}");

        // Get route by id
        let req = test::TestRequest::get()
            .uri(&format!("/routes/{id}"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body: Route = test::read_body_json(resp).await;
        println!("returned Route for funky monkey:\n{:?}", body);

        // Update route by id
        let mut updated_route = test_route();
        updated_route.difficulty = DifficultyRating::Rating512;
        let req = test::TestRequest::put()
            .uri(&format!("/routes/{id}"))
            .set_json(updated_route)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Get route by id again to verify that the change worked
        let req = test::TestRequest::get()
            .uri(&format!("/routes/{id}"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body: Route = test::read_body_json(resp).await;
        println!(
            "returned Route for funky monkey after updating:\n{:?}",
            body
        );
        assert_eq!(body.difficulty, DifficultyRating::Rating512);

        // Delete route
        let req = test::TestRequest::delete()
            .uri(&format!("/routes/{id}"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Get route by id again to verify that the deletion worked
        let req = test::TestRequest::get()
            .uri(&format!("/routes/{id}"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
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

    #[actix_web::test]
    async fn test_get_routes_with_specific_number() {
        let app = test::init_service(app!()).await;
        let num_routes = NumberRoutes { number_routes: 2 };
        let req = test::TestRequest::get()
            .uri("/routes")
            .set_json(num_routes)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body: Vec<Route> = test::read_body_json(resp).await;
        println!("GET /routes with number_routes = 2 response:\n{:?}", body);
    }
}
