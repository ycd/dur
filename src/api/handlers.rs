use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

use crate::Memory;

#[derive(Serialize)]
struct Health<T, V>
where
    T: Into<String>,
    V: Into<String>,
{
    status: T,
    version: V,
}

#[get("/health")]
pub async fn get_health() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(Health {
            status: "ok",
            version: env!("CARGO_PKG_VERSION"),
        })
}

#[derive(Serialize)]
struct LimitResponse {
    allowed: bool,
}

#[get("/request/{id}")]
pub async fn new_request(
    path: web::Path<(u64,)>,
    data: web::Data<Mutex<crate::Dur<Memory>>>,
) -> impl Responder {
    let mut _data = data.lock().unwrap();
    let id = path.into_inner().0;

    let allowed = _data.request(id, None);

    let remaning_requests: i32 = _data.config.limit() as i32 - _data.remaning_requests(id) as i32;

    HttpResponse::Ok()
        .json(LimitResponse { allowed: allowed })
        .with_header("X-Ratelimit-Remaning", remaning_requests.to_string())
        .with_header("X-Ratelimit-Limit", _data.config.limit().to_string())
}
