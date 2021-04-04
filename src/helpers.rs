use actix_web::{web::Json, ResponseError};
use serde::Serialize;

/// Helper function to reduce boilerplate of an OK/Json response
pub fn respond_json<T>(data: T) -> Result<Json<T>, Box<dyn ResponseError>>
where
    T: Serialize,
{
    Ok(Json(data))
}
