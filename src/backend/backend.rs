use std::{error::Error, time::Duration};

// Use the selected backend for storing related information
// about the API.
pub trait Backend {
    fn new() -> Self;
    fn clear(&mut self);
    fn evict_older_timestamps(&mut self, id: u64, timestamp: Duration, window_time: u16);
    fn insert(
        &mut self,
        id: u64,
        ip_addr: Option<std::net::IpAddr>,
    ) -> Result<usize, Box<dyn Error>>;
    fn len(&self) -> usize;
    fn request_count(&mut self, id: u64) -> usize;
}
