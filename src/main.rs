extern crate dotenvy;

use std::{env, io};

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::{Logger, NormalizePath};
use dotenvy::dotenv;
use log::info;
use tracing_actix_web::TracingLogger;

// External modules reference
mod router;
mod logger;

#[actix_web::main] // or #[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    logger::init();

    let app_port = env::var("APP_PORT").expect("APP_PORT env not set.");
    info!("Starting HTTP server at http://localhost:{}", app_port);

    HttpServer::new(move || {
        let logger = Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T %D"#);
        let tracing = TracingLogger::default();
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(NormalizePath::new(Default::default()))
            .wrap(tracing)
            .wrap(logger)
            .wrap(cors)
            .configure(router::init)
            .default_service(web::route().to(not_found))
    })
        .bind(&format!("0.0.0.0:{}", app_port))?
        .run()
        .await
}

/// 404 Not Found
async fn not_found() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>")
}
