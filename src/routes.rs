/// Functions for the API endpoint /routes
use crate::pg::conn;
use crate::route::Route;
use actix_web::{post, web, HttpResponse, Responder};
use dotenvy::dotenv;
use sqlx::{self, query};

#[post("/routes")]
async fn routes_post(json: web::Json<Route>) -> impl Responder {
    // println!("routes_post with route json = {:?}", json);
    // let route = json.0;
    // println!("route = {:?}", route);
    if let Ok(mut conn) = conn().await {
        dotenv().ok();
        if let Ok(_query_result) = query!(
            "INSERT INTO routes (name, difficulty, latitude, longitude) VALUES ($1, $2, $3, $4)",
            json.0.name,
            format!("{}", json.0.difficulty),
            json.0.latitude,
            json.0.longitude
        )
        .execute(&mut conn)
        .await
        {
            HttpResponse::Ok()
        } else {
            HttpResponse::BadGateway() // established connection to DB but INSERT failed
        }
    } else {
        HttpResponse::BadGateway() // failed to connect to db
    }
}
