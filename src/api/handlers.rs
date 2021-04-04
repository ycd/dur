use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;

use crate::{Backend, Memory};

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
    data: web::Data<Arc<Mutex<crate::Dur<Memory>>>>,
) -> impl Responder {
    let mut _data = data.lock().unwrap();
    let id = path.into_inner().0;

    let allowed = _data.request(id, None);

    let remaning_requests = _data.remaning_requests(id);

    HttpResponse::Ok()
        .json(LimitResponse { allowed: allowed })
        .with_header("X-Ratelimit-Remaning", remaning_requests.to_string())
        .with_header("X-Ratelimit-Limit", _data.config.limit().to_string())
}
