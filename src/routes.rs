use crate::pg::conn;
/// "business logic" for the API endpoint /routes
use crate::route::Route;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::{self, query};
use dotenvy::dotenv;

#[post("/routes")]
async fn routes_post(json: web::Json<Route>) -> impl Responder {
    // println!("routes_post with route json = {:?}", json);
    // let route = json.0;
    // println!("route = {:?}", route);
    let conn = conn().await;

    dotenv().ok(); 
    let query_result = query!(
        "INSERT INTO routes (name, difficulty, location) VALUES ($1, $2, $3)",
        json.0.name,
        json.0.difficulty,
        json.0.location,
    );
    HttpResponse::Ok()
}
