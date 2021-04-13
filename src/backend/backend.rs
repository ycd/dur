use std::{error::Error, net::Ipv4Addr, time::Duration};

use super::IpAndPath;

// Use the selected backend for storing related information
// about the API.
pub trait Backend {
    fn new() -> Self;
    fn clear(&mut self);
    fn evict_older_timestamps(&mut self, id: u64, timestamp: Duration, window_time: u16);
    fn insert(&mut self, id: u64, ip_and_path: IpAndPath) -> Result<usize, Box<dyn Error>>;
    fn len(&self) -> usize;
    fn request_count(&self, id: u64) -> usize;
    fn ip_address_count(&self, id: u64, ip: Ipv4Addr) -> usize;
    fn path_count(&self, id: u64, path: String) -> usize;
}
