mod delete;
mod env;
mod get;
mod post;
mod put;
mod utils;

use crate::delete::delete_responder;
use crate::env::{app_port, load_config};
use crate::get::{get_responder, health_check};
use crate::post::post_responder;
use crate::put::put_responder;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_config();

    println!("Starting  application...");
    println!("Server running on PORT {:?}", app_port());
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allowed_headers(vec!["content-type", "authorization"])
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(health_check)
            .service(get_responder)
            .service(post_responder)
            .service(put_responder)
            .service(delete_responder)
    })
    .bind(("0.0.0.0", app_port()))?
    .run()
    .await
}
