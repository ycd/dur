use std::{net::Ipv4Addr, str::FromStr, sync::Mutex};

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{IpAndPath, Memory};

#[derive(Serialize)]
struct Health<T>
where
    T: Into<String>,
{
    status: T,
}

#[get("/health")]
pub async fn get_health() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(Health { status: "ok" })
}

#[derive(Serialize)]
struct LimitResponse {
    allowed: bool,
    metadata: Metadata,
}

#[derive(Serialize)]
struct Metadata {
    id: u64,
    x_ratelimit_remaning: i32,
    x_ratelimit_limit: u32,
    path: Option<String>,
    ip: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    id: u64,
    path: Option<String>,
    ip: Option<String>,
}

#[derive(Serialize)]
pub struct BadRequest {
    error: String,
}

#[post("/request")]
pub async fn new_request(
    payload: web::Json<Request>,
    data: web::Data<Mutex<crate::Dur<Memory>>>,
) -> impl Responder {
    let mut _data = data.lock().unwrap();

    let ip_addr: Option<Ipv4Addr> = match payload.ip {
        None => None,
        Some(ref v) => match Ipv4Addr::from_str(v) {
            Ok(ip) => Some(ip),
            Err(_) => {
                return HttpResponse::BadRequest()
                    .json(BadRequest {
                        error: format!("invalid ip address: {}", v),
                    })
                    .with_header("X-Ratelimit-Limit", _data.config.limit() as usize)
            }
        },
    };

    let request = _data.request(payload.id, IpAndPath::new(ip_addr, payload.path.clone()));
    let remaning_requests: i32 = _data.config.limit() as i32 - request.1 as i32;

    HttpResponse::Ok()
        .json(LimitResponse {
            allowed: request.0,
            metadata: Metadata {
                x_ratelimit_remaning: remaning_requests,
                x_ratelimit_limit: _data.config.limit(),
                id: payload.id,
                path: payload.path.clone(),
                ip: payload.ip.clone(),
            },
        })
        .with_header("X-Ratelimit-Remaning", remaning_requests as usize)
        .with_header("X-Ratelimit-Limit", _data.config.limit() as usize)
}
