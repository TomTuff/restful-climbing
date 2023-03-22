use crate::pg::conn;
/// "business logic" for the API endpoint /routes
use crate::route::Route;
use actix_web::{post, web, HttpResponse, Responder};
use dotenvy::dotenv;
use sqlx::{self, query};

#[post("/routes")]
async fn routes_post(json: web::Json<Route>) -> impl Responder {
    // println!("routes_post with route json = {:?}", json);
    // let route = json.0;
    // println!("route = {:?}", route);
    let mut conn = conn().await;

    dotenv().ok();
    let mut query_result = query!(
        "INSERT INTO routes (name, difficulty, latitude, longitude) VALUES ($1, $2, $3, $4)",
        json.0.name,
        format!("{}", json.0.difficulty),
        json.0.latitude,
        json.0.longitude
    )
    .execute(&mut conn)
    .await;

    HttpResponse::Ok()
}
