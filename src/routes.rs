use crate::error::DatabaseError;
use crate::pg::conn;
use crate::route::{DifficultyRating, Route, NumberRoutes};
use actix_web::{get, post, delete, put, web, HttpResponse, Responder};
use log::error;
use sqlx::{self, query};
/// Functions for the API endpoint /routes
use std::str::FromStr;

#[post("")]
async fn add_new_route(json: web::Json<Route>) -> impl Responder {
    if let Ok(mut conn) = conn().await {
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
            error!("INSERT query failed in add_new_route()");
            HttpResponse::BadGateway() 
        }
    } else {
        error!("Failed to connect to the database in add_new_route()");
        HttpResponse::BadGateway() 
    }
}

#[get("")]
async fn get_recent_routes(number_routes: Option<web::Json<NumberRoutes>>) -> impl Responder {
    let number_of_routes_to_request = match number_routes {
        Some(n) => { n.0.number_routes }
        None => { 5 }
    };
    if let Ok(mut conn) = conn().await {
        if let Ok(query_result) = query!(
            r#"SELECT id, name as "name!", difficulty as "difficulty!", latitude as "latitude!", longitude as "longitude!" FROM routes ORDER BY created_at DESC LIMIT ($1)"#,
            number_of_routes_to_request,
        )
            .fetch_all(&mut conn)
            .await
        {
            if let Ok(routes) = query_result
                .iter()
                .map(|record| {
                    Ok(Route::new(
                        Some(record.id),
                        record.name.to_owned(),
                        DifficultyRating::from_str(&record.difficulty)?,
                        record.latitude,
                        record.longitude,
                    ))
                })
                .collect::<Result<Vec<Route>, DatabaseError>>() {
                    HttpResponse::Ok().json(routes)
            } else {
                error!("Failed to parse difficulty column to a DifficultyRating in get_recent_routes()");
                HttpResponse::BadGateway().finish()
            }
        } else {
            error!("SELECT query failed in get_recent_routes()");
            HttpResponse::BadGateway().finish()
        }
    } else {
        error!("Failed to connect to the database in get_recent_routes()");
        HttpResponse::BadGateway().finish()
    }
}

#[get("/{id}")]
async fn get_route_by_id(path: web::Path<(i32)>) -> impl Responder {
    let id = path.into_inner();
    if let Ok(mut conn) = conn().await {
        if let Ok(query_result) = query!(
            r#"SELECT id, name as "name!", difficulty as "difficulty!", latitude as "latitude!", longitude as "longitude!" FROM routes WHERE id = ($1)"#,
            id,
        )
            .fetch_one(&mut conn)
            .await
        {
            if let Ok(difficulty_rating) = DifficultyRating::from_str(&query_result.difficulty) {
                let route = Route::new(
                    Some(query_result.id),
                    query_result.name.to_owned(),
                    difficulty_rating,
                    query_result.latitude,
                    query_result.longitude,
                );
                HttpResponse::Ok().json(route)
            } else {
                error!("Failed to parse difficulty column to a DifficultyRating in get_route_by_id()");
                HttpResponse::BadGateway().finish()
            }
        } else {
            error!("SELECT query failed in get_route_by_id()");
            HttpResponse::BadGateway().finish()
        }
    } else {
        error!("Failed to connect to the database in get_route_by_id()");
        HttpResponse::BadGateway().finish()
    }
}

#[delete("/{id}")]
async fn delete_route_by_id() -> impl Responder {
    HttpResponse::ExpectationFailed()
}

#[put("/{id}")]
async fn update_route_by_id() -> impl Responder {
    HttpResponse::ExpectationFailed()
}
