pub struct Config {
    // The maximum limit for a user with the given id
    // can send maximum request in a single period.
    limit: u32,

    // Maximum count of unique IP addresses
    // that the same user can send requests from
    // in a single period.
    ip_addr_limit: u16,
}

impl Config {
    pub fn new(limit: Option<u32>, ip_addr_limit: Option<u16>) -> Self {
        Self {
            limit: limit.unwrap_or(50 as u32),
            ip_addr_limit: ip_addr_limit.unwrap_or(16 as u16),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            limit: 50 as u32,
            ip_addr_limit: 5 as u16,
        }
    }
}
