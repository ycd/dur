mod api;
mod backend;
mod config;
mod dur;
mod errors;
mod helpers;

use std::sync::Mutex;

pub use backend::{Backend, Memory};
pub use config::Config;

use actix_web::{web, App, HttpServer};

pub use dur::Dur;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Mutex::new(Dur::new(Memory::new(), None)));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(api::get_health)
            .service(api::new_request)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
