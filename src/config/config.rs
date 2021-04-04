#[derive(Debug, Clone)]
pub struct Config {
    // The maximum limit for a user with the given id
    // can send maximum request in a single period.
    limit: u32,

    // Maximum count of unique IP addresses
    // that the same user can send requests from
    // in a single period.
    ip_addr_limit: u16,

    // Window time in seconds.
    window_time: u16,
}

impl Config {
    pub fn new(limit: Option<u32>, ip_addr_limit: Option<u16>, window_time: Option<u16>) -> Self {
        Self {
            limit: limit.unwrap_or(50 as u32),
            ip_addr_limit: ip_addr_limit.unwrap_or(16 as u16),
            window_time: window_time.unwrap_or(300 as u16),
        }
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }

    pub fn set_limit(&mut self, limit: u32) -> u32 {
        self.limit = limit;

        self.limit
    }

    pub fn ip_addr_limit(&self) -> u16 {
        self.ip_addr_limit
    }

    pub fn set_ip_addr_limit(&mut self, limit: u16) -> u16 {
        self.ip_addr_limit = limit;

        self.ip_addr_limit
    }

    pub fn window_time(&self) -> u16 {
        self.window_time
    }

    pub fn set_window_time(&mut self, window_time: u16) -> u16 {
        self.window_time = window_time;

        self.window_time
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            limit: 50 as u32,
            ip_addr_limit: 5 as u16,
            window_time: 300 as u16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updating_window_log() {
        let mut config = Config::default();

        assert!(config.limit() == 50);
        assert!(config.ip_addr_limit() == 5);
        assert!(config.window_time() == 300);

        config.set_window_time(500);
        config.set_limit(100);
        config.set_ip_addr_limit(25);

        assert!(config.window_time() == 500);
        assert!(config.limit() == 100);
        assert!(config.ip_addr_limit() == 25);
    }
}
