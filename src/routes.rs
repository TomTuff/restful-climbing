use crate::pg::conn;
/// "business logic" for the API endpoint /routes
use crate::route::Route;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx;

#[post("/routes")]
async fn routes_post(json: web::Json<Route>) -> impl Responder {
    // println!("routes_post with route json = {:?}", json);
    // let route = json.0;
    // println!("route = {:?}", route);
    let conn = conn().await;
    HttpResponse::Ok()
}
