use actix_files::Files;
use actix_web::{web::{Data}, App, HttpServer};
use tera::Tera;

use  std::io::{Result};

mod handlers;

#[actix_web::main]
async fn main() -> Result<()> {
    unsafe  {
    std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("Starting server at http://localhost:8080");
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tera.clone()))
            .service(handlers::index)
            .service(handlers::projects)
            .service(handlers::contact)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}