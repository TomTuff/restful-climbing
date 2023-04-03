use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;

pub mod climb;
pub mod climber;
mod climbers;
pub mod error;
pub mod pg;
pub mod route;
mod routes;

#[cfg(test)]
mod tests;

// macro for generating the app so that we don't have redundant code in tests module and main()
#[macro_export]
macro_rules! app (
    () => ({
        dotenv().ok();
        let _ = env_logger::try_init_from_env(Env::default().default_filter_or("info"));  // assign to _ because Result<(), SetLoggerError> is intentionally unused; SetLoggerError indicates set_logger was already called, which is fine.
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/routes")
                    .service(routes::add_new_route)
                    .service(routes::get_recent_routes)
                    .service(routes::get_route_by_id)
                    .service(routes::delete_route_by_id)
                    .service(routes::update_route_by_id)
            )
            .service(
                web::scope("/climbers")
                    .service(climbers::get_recent_climbers)
                    .service(climbers::add_new_climber)
                    .service(climbers::get_climber_recent_climbs)
                    .service(climbers::delete_climber)
                    .service(climbers::get_climbers_review_by_route_id)
                    .service(climbers::add_review)
                    .service(climbers::update_review)
                    .service(climbers::delete_review)
            )
    });
);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| app!())
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
