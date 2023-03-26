use crate::error::DatabaseError;
use crate::pg::conn;
use crate::route::{DifficultyRating, Route};
use actix_web::{get, post, web, HttpResponse, Responder};
use dotenvy::dotenv;
use sqlx::{self, query};
/// Functions for the API endpoint /routes
use std::str::FromStr;

#[post("/routes")]
async fn routes_post(json: web::Json<Route>) -> impl Responder {
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

#[get("/routes")]
async fn routes_get() -> impl Responder {
    if let Ok(mut conn) = conn().await {
        dotenv().ok();
        if let Ok(query_result) = query!(r#"SELECT name as "name!", difficulty as "difficulty!", latitude as "latitude!", longitude as "longitude!" FROM routes ORDER BY created_at DESC LIMIT 5"#)
            .fetch_all(&mut conn)
            .await
        {
            if let Ok(routes) = query_result
                .iter()
                .map(|record| {
                    Ok(Route::new(
                        record.name.to_owned(),
                        DifficultyRating::from_str(&record.difficulty)?,
                        record.latitude,
                        record.longitude,
                    ))
                })
                .collect::<Result<Vec<Route>, DatabaseError>>() {
                    HttpResponse::Ok().json(routes)
            } else {
                HttpResponse::BadGateway().finish()  // failed to parse difficulty column to a DifficultyRating
            }
        } else {
            HttpResponse::BadGateway().finish() // established connection to DB but SELECT failed
        }
    } else {
        HttpResponse::BadGateway().finish() // failed to connect to db
    }
}
