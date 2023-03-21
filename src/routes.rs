/// "business logic" for the API endpoint /routes
use crate::route::Route;
use sqlx;
use actix_web::{Responder, HttpResponse, post};

#[post("/routes")]
async fn routes_post() -> impl Responder {
    // TODO: add SQL
    HttpResponse::Ok()
}