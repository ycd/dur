use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Ip {
    ip_addresses: Option<Vec<String>>,
    limit: Option<u16>,
    window_time: Option<u16>,
}

impl Ip {
    pub fn new<I, T>(endpoints: I, limit: u16, window_time: u16) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            ip_addresses: Some(endpoints.into_iter().map(Into::into).collect()),
            limit: Some(limit),
            window_time: Some(window_time),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_new() {
        let ip = Ip::new(vec!["test", "1234", "214141"], 300, 400);

        assert_eq!(ip.ip_addresses.clone().unwrap()[0], "test".to_owned());
        assert_eq!(ip.ip_addresses.clone().unwrap()[1], "1234".to_owned());
        assert_eq!(ip.ip_addresses.clone().unwrap()[2], "214141".to_owned());
        assert_eq!(ip.limit, Some(300));
        assert_eq!(ip.window_time, Some(400));
    }
}
