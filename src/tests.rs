use crate::{
    climb::{Climb, Rating, Review},
    climber::{Climber, NumberClimbers},
    route::{DifficultyRating, NumberRoutes, Route},
};

use super::*;
use actix_web::{
    http::{self},
    test,
};
use chrono::NaiveDate;

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

#[actix_web::test]
async fn test_add_get_delete_climber() {
    let app = test::init_service(app!()).await;

    let test_climber = Climber {
        id: None,
        username: "testclimber123".to_string(),
    };

    // Add climber
    let req = test::TestRequest::post()
        .uri("/climbers")
        .set_json(test_climber)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    // Get climber
    let one_climber = NumberClimbers { number_climbers: 1 };
    let req = test::TestRequest::get()
        .uri("/climbers")
        .set_json(one_climber)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    let body: Vec<Climber> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    let id = body[0].id.unwrap();
    println!("returned id for test climber: {id}");

    // Get climber by id
    let req = test::TestRequest::get()
        .uri(&format!("/climbers/{id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    let body: Climber = test::read_body_json(resp).await;
    println!("returned climbs for test climber:\n{:?}", body);

    // Delete climber by id
    let req = test::TestRequest::delete()
        .uri(&format!("/climbers/{id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    // Get climber by id again to verify that the deletion worked
    let req = test::TestRequest::get()
        .uri(&format!("/climbers/{id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_post_get_put_delete_climb() {
    let app = test::init_service(app!()).await;

    let test_climber = Climber {
        id: None,
        username: "testclimber456".to_string(),
    };

    // Add climber
    let req = test::TestRequest::post()
        .uri("/climbers")
        .set_json(test_climber)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    // Get climber id
    let one_climber = NumberClimbers { number_climbers: 1 };
    let req = test::TestRequest::get()
        .uri("/climbers")
        .set_json(one_climber)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    let body: Vec<Climber> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    let climber_id = body[0].id.unwrap();
    println!("returned climber_id for test climber: {climber_id}");

    // Add route
    let req = test::TestRequest::post()
        .uri("/routes")
        .set_json(test_route())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    // Get route id
    let one_route = NumberRoutes { number_routes: 1 };
    let req = test::TestRequest::get()
        .uri("/routes")
        .set_json(one_route)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    let body: Vec<Route> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    let route_id = body[0].id.unwrap();
    println!("returned route_id for funky monkey: {route_id}");

    // add test review
    let test_review = Review::new(
        10,
        "I loved this route!".to_string(),
        NaiveDate::from_ymd_opt(2023, 4, 2).unwrap(),
    );
    let req = test::TestRequest::post()
        .uri(&format!("/climbers/{climber_id}/{route_id}"))
        .set_json(test_review)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    // get that test review
    let req = test::TestRequest::get()
        .uri(&format!("/climbers/{climber_id}/{route_id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    let body: Climb = test::read_body_json(resp).await;
    assert_eq!(body.climber_id, climber_id);
    assert_eq!(body.route_id, route_id);
    let climb_id = body.id.unwrap();

    // update that test review
    let updated_test_review = Review::new(
        2,
        "Actually, I hated it!".to_string(),
        NaiveDate::from_ymd_opt(2023, 4, 2).unwrap(),
    );
    let req = test::TestRequest::put()
        .uri(&format!("/climbers/{climber_id}/{route_id}"))
        .set_json(updated_test_review)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    // delete that test review
    let req = test::TestRequest::delete()
        .uri(&format!("/climbers/{climber_id}/{route_id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    // Delete route
    let req = test::TestRequest::delete()
        .uri(&format!("/routes/{route_id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    // Delete climber by id
    let req = test::TestRequest::delete()
        .uri(&format!("/climbers/{climber_id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}
