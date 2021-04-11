mod api;
mod backend;
mod client;
mod config;
mod dur;
mod helpers;

use std::sync::Mutex;

pub use backend::{Backend, IpAndPath, Memory};
pub use config::Config;

use actix_web::{web, App, HttpServer};

pub use dur::Dur;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = client::cli();

    let data = web::Data::new(Mutex::new(Dur::new(Memory::new(), Some(config.clone()))));

    eprintln!("dur is running on: {}", &config.host_and_port());
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(api::get_health)
            .service(api::new_request)
    })
    .bind(&config.host_and_port())?
    .run()
    .await
}
