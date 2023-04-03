use crate::climb::{Climb, Review};
use crate::climber::{Climber, NumberClimbers};
use crate::pg::conn;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::error;
use sqlx::{self, query};

#[get("")]
async fn get_recent_climbers(number_climbers: Option<web::Json<NumberClimbers>>) -> impl Responder {
    let number_of_climbers_to_request = match number_climbers {
        Some(n) => n.0.number_climbers,
        None => 5,
    };

    if let Ok(mut conn) = conn().await {
        if let Ok(query_result) = query!(
            r#"SELECT id as "id!", username as "username!" FROM climbers ORDER BY created_at DESC LIMIT ($1)"#,
            number_of_climbers_to_request,
        )
        .fetch_all(&mut conn)
        .await
        {
            let climbers = query_result
                .iter()
                .map(|record| Climber {
                    id: Some(record.id),
                    username: record.username.to_owned(),
                })
                .collect::<Vec<Climber>>();
            HttpResponse::Ok().json(climbers)
        } else {
            error!("SELECT query failed in get_recent_climbers()");
            HttpResponse::BadGateway().finish()
        }
    } else {
        error!("Failed to connect to the database in get_recent_climbers()");
        HttpResponse::BadGateway().finish()
    }
}

#[post("")]
async fn add_new_climber(json: web::Json<Climber>) -> impl Responder {
    if let Ok(mut conn) = conn().await {
        if let Ok(_query_result) = query!(
            "INSERT INTO climbers (username) VALUES ($1)",
            json.0.username,
        )
        .execute(&mut conn)
        .await
        {
            HttpResponse::Ok()
        } else {
            error!("INSERT query failed in add_new_climber()");
            HttpResponse::BadGateway()
        }
    } else {
        error!("Failed to connect to the database in add_new_climber()");
        HttpResponse::BadGateway()
    }
}

#[get("/{id}")]
async fn get_climber_recent_climbs(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    if let Ok(mut conn) = conn().await {
        if let Ok(query_result) = query!(
            r#"SELECT id, username as "username!" FROM climbers WHERE id = ($1)"#,
            id
        )
        .fetch_one(&mut conn)
        .await
        {
            let climber = Climber::new(Some(query_result.id), query_result.username);
            HttpResponse::Ok().json(climber)
        } else {
            error!("SELECT query failed in get_climber_recent_climbs()");
            HttpResponse::BadRequest().finish()
        }
    } else {
        error!("Failed to connect to the database in get_climber_recent_climbs()");
        HttpResponse::BadGateway().finish()
    }
}

#[delete("/{id}")]
async fn delete_climber(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    if let Ok(mut conn) = conn().await {
        if query!(r#"DELETE FROM climbers WHERE id = ($1)"#, id,)
            .execute(&mut conn)
            .await
            .is_ok()
        {
            HttpResponse::Ok()
        } else {
            error!("DELETE query failed in delete_climber()");
            HttpResponse::BadGateway()
        }
    } else {
        error!("Failed to connect to the database in delete_climber()");
        HttpResponse::BadGateway()
    }
}

#[get("/{climber_id}/{route_id}")]
async fn get_climbers_review_by_route_id(path: web::Path<(i32, i32)>) -> impl Responder {
    let (climber_id, route_id) = path.into_inner();
    if let Ok(mut conn) = conn().await {
        if let Ok(query_result) = query!(
            r#"SELECT id, climber_id as "climber_id!", route_id as "route_id!", rating as "rating!", review as "review!", completion_date as "completion_date!" FROM climbs WHERE climber_id = ($1) and route_id = ($2)"#,
            climber_id,
            route_id,
        )
        .fetch_one(&mut conn)
        .await
        {
            let climb = Climb {
                id: Some(query_result.id),
                climber_id: query_result.climber_id,
                route_id: query_result.route_id,
                review: Review::new(
                    query_result.rating,
                    query_result.review,
                    query_result.completion_date,
                )                
            };
            HttpResponse::Ok().json(climb)
        } else {
            error!("INSERT query failed in add_review()");
            HttpResponse::BadRequest().finish()
        }
    } else {
        error!("Failed to connect to the database in add_review()");
        HttpResponse::BadGateway().finish()
    }

}

#[post("/{climber_id}/{route_id}")]
async fn add_review(path: web::Path<(i32, i32)>, json: web::Json<Review>) -> impl Responder {
    let (climber_id, route_id) = path.into_inner();
    if let Ok(mut conn) = conn().await {
        if query!(
            r#"INSERT INTO climbs (climber_id, route_id, rating, review, completion_date) VALUES ($1, $2, $3, $4, $5)"#,
            climber_id,
            route_id,
            json.0.rating.i32(),
            json.0.review,
            json.0.completion_date,
        )
        .execute(&mut conn)
        .await
        .is_ok()
        {
            HttpResponse::Ok().finish()
        } else {
            error!("INSERT query failed in add_review()");
            HttpResponse::BadRequest().finish()
        }
    } else {
        error!("Failed to connect to the database in add_review()");
        HttpResponse::BadGateway().finish()
    }
}

#[put("/{climber_id}/{route_id}")]
async fn update_review(path: web::Path<(i32, i32)>, json: web::Json<Review>) -> impl Responder {
    let (climber_id, route_id) = path.into_inner();
    if let Ok(mut conn) = conn().await {
        if query!(
            r#"UPDATE climbs SET rating = $1, review = $2, completion_date = $3 WHERE climber_id = ($4) AND route_id = ($5)"#,
            json.0.rating.i32(),
            json.0.review,
            json.0.completion_date,
            climber_id,
            route_id,
        )
        .execute(&mut conn)
        .await
        .is_ok()
        {
            HttpResponse::Ok().finish()
        } else {
            error!("INSERT query failed in add_review()");
            HttpResponse::BadRequest().finish()
        }
    } else {
        error!("Failed to connect to the database in add_review()");
        HttpResponse::BadGateway().finish()
    }
}

#[delete("/{climber_id}/{route_id}")]
async fn delete_review() -> impl Responder {
    HttpResponse::ExpectationFailed()
}
