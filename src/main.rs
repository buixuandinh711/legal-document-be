mod app_config;
mod models;
mod routes;

use crate::routes::{auth::auth_routes, home::home_routes};
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    middleware, web, App, HttpServer,
};
use dotenv::dotenv;
use tokio_postgres::NoTls;
const ONE_MINUTE: Duration = Duration::minutes(1);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let secret_key = Key::generate();
    let app_config = app_config::AppConfig::from_env().unwrap();

    let db_pool = app_config.pg.create_pool(None, NoTls).unwrap();
    let _ = db_pool.get().await.unwrap(); // panic if unable to connect
    log::info!("Database connected!");

    log::info!("Server started at: {}", &app_config.server_addr);
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(ONE_MINUTE))
                    .build(),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(db_pool.clone()))
            .configure(auth_routes)
            .configure(home_routes)
    })
    .bind(&app_config.server_addr)?
    .run()
    .await
}
