use actix_files::Files;
use actix_web::{web::{Data}, App, HttpServer};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use tera::Tera;
use  std::io::{Result};

mod handlers;

#[actix_web::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize database
    let db_pool = handlers::init_db()
        .await
        .expect("Failed to initialize database");

    let tera = Tera::new("templates/**/*").unwrap();
    
    // Generate a secret key for sessions (64 bytes: 32 for signing + 32 for encryption)
    let secret_key = Key::from(&[0u8; 64]);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tera.clone()))
            .app_data(Data::new(db_pool.clone()))
            .wrap(SessionMiddleware::builder(
                CookieSessionStore::default(),
                secret_key.clone()
            )
            .cookie_name("portfolio_session".to_string())
            .cookie_secure(false) // Set to true in production with HTTPS
            .cookie_http_only(true)
            .build())
            .service(Files::new("/static", "static").show_files_listing())
            .service(handlers::index)
            .service(handlers::dashboard)
            .service(handlers::login_page)
            .service(handlers::login)
            .service(handlers::logout)
            .service(handlers::submit_contact)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
