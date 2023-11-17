mod db;
mod middleware;
mod models;
mod routers;
mod schema;
mod oauth;
mod config;
mod jwt;
mod worklog;

use middleware::default_handler;
use config::Config;
use actix_cors::Cors;
use actix_web::{middleware::{ErrorHandlers, Logger, NormalizePath}, web, App, HttpServer, http};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = Config::init();
    let server_address = config.server_address.clone();
    let pool = db::initialize_db_pool(&config.database_url);
    
    db::run_migrations(pool.clone());

    let oauth_client = oauth::initialize_oauth_client(&config.clone());

    HttpServer::new(move || {
        let cors = Cors::default()
            // .allowed_origin(&*config.front_url)
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(ErrorHandlers::new()
                .default_handler(default_handler))
            .wrap(NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(oauth_client.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(routers::auth::auth)
                    .service(routers::auth::auth_callback)
                    .service(routers::auth::refresh)
                    .service(routers::worklog::worklog)
                    .service(routers::worklog::delete_worklog)
            )
    })
        .workers(5)
        .bind(server_address)?
        .run()
        .await
}
