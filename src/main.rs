mod app_config;
mod models;
mod routes;
mod utils;

use crate::{
    app_config::{AppState, CloudStorage},
    routes::{
        draft_route::draft_routes, home::home_routes, officer_route::auth_routes,
        published_doc_route::published_doc_routes,
    },
};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    middleware, web, App, HttpServer,
};
use cloud_storage::Client;
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let secret_key = Key::generate();
    let app_config = app_config::AppConfig::from_env().unwrap();

    let db_pool = app_config.pg.create_pool(None, NoTls).unwrap();
    {
        let _ = db_pool.get().await.unwrap();
    } // panic if unable to connect
    log::info!("Database connected!");

    let bucket_name = dotenv::var("CLOUD_STORAGE_BUCKET").unwrap();
    let cloud_storage_base_url = dotenv::var("CLOUD_STORAGE_BASE_URL").unwrap();

    log::info!("Server started at: {}", &app_config.server_addr);
    HttpServer::new(move || {
        let cors = Cors::permissive();

        let app_state = AppState {
            db_pool: db_pool.clone(),
            cloud_storage: CloudStorage {
                client: Client::default(),
                bucket_name: bucket_name.clone(),
                base_url: cloud_storage_base_url.clone(),
            },
        };

        App::new()
            .wrap(cors)
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::WEEK))
                    .build(),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(app_state))
            .configure(auth_routes)
            .configure(home_routes)
            .configure(draft_routes)
            .configure(published_doc_routes)
    })
    .workers(2)
    .bind(&app_config.server_addr)?
    .run()
    .await
}
