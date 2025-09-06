use actix_files::Files;
use actix_web::{web::{Data}, App, HttpServer};
use tera::Tera;

use  std::io::{Result};

mod handlers;

#[actix_web::main]
async fn main() -> Result<()> {
    // unsafe  {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // }
    // env_logger::init();

    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tera.clone()))
            .service(handlers::index)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}