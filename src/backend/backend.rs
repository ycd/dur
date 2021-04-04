use std::error::Error;

// Use the selected backend for storing related information
// about the API.
pub trait Backend {
    fn new() -> Self;
    fn clear(&mut self);
    fn insert(
        &mut self,
        id: i64,
        ip_addr: Option<std::net::IpAddr>,
    ) -> Result<usize, Box<dyn Error>>;
    fn len(&self) -> usize;
    fn request_count(&mut self, id: i64) -> usize;
}
