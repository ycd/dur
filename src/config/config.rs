pub struct Config {
    limit: u32,
}

impl Config {
    pub fn new(limit: Option<u32>) -> Self {
        Self {
            limit: limit.unwrap_or(50 as u32),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { limit: 50 as u32 }
    }
}
