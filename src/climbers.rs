use crate::pg::conn;
use crate::climber::{Climber, NumberClimbers};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::error;
use sqlx::{self, query};

#[get("")]
async fn get_recent_climbers() -> impl Responder {
    HttpResponse::ExpectationFailed()
}

#[post("")]
async fn add_new_climber() -> impl Responder {
    HttpResponse::ExpectationFailed()
}

#[get("/climbers/{id}")]
async fn get_climber_recent_climbs() -> impl Responder {
    HttpResponse::ExpectationFailed()
}

#[delete("/climbers/{id}")]
async fn delete_climber() -> impl Responder {
    HttpResponse::ExpectationFailed()
}

#[get("/climbers/{climber_id}/{route_id}")]
async fn get_climbers_review_by_route_id() -> impl Responder {
    HttpResponse::ExpectationFailed()
}

#[post("/climbers/{climber_id}/{route_id}")]
async fn add_review() -> impl Responder {
    HttpResponse::ExpectationFailed()
}

#[put("/climbers/{climber_id}/{route_id}")]
async fn update_review() -> impl Responder {
    HttpResponse::ExpectationFailed()
}

#[delete("/climbers/{climber_id}/{route_id}")]
async fn delete_review() -> impl Responder {
    HttpResponse::ExpectationFailed()
}
