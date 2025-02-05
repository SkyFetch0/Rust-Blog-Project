mod models;
mod routes;
mod handlers;
mod templates;
mod filters;
mod state;

use actix_web::{App, HttpServer, web, middleware};
use actix_files as fs;
use actix_cors::Cors;
use dotenv::dotenv;
use std::time::Duration;
use state::AppState;
use moka::future::Cache;
use std::sync::Arc;
use crate::routes::configure_routes;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = loop {
        match models::db::init_db().await {
            Ok(pool) => {
                println!("Database connection successful!");
                break pool;
            }
            Err(e) => {
                println!("Database connection failed: {:?}. Retrying in 5 seconds...", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        }
    };

<<<<<<< HEAD
    let post_cache = Arc::new(
        Cache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_secs(120))
            .build(),
    );

    let app_state = AppState {
        pool: pool.clone(),
        post_cache,
    };

    println!("Server running at http://localhost:8080");
=======
    println!("Server running at http://0.0.0.0:8080");
>>>>>>> 08f48e64cb276e8ffb27696f1ab422df34a008a2

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            // Artık sadece pool değil, tüm AppState'i ekliyoruz:
            .app_data(web::Data::new(app_state.clone()))
            .service(
                fs::Files::new("/static", "static")
                    .show_files_listing()
                    .use_last_modified(true)
                    .use_etag(true)
                    .prefer_utf8(true)
            )
            .configure(routes::configure_routes)
    })
        .workers(num_cpus::get() * 2)
        .backlog(1024)
        .keep_alive(Duration::from_secs(30))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
