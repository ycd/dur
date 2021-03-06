use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Path {
    paths: Option<Vec<String>>,
    limit: Option<u32>,
    window_time: Option<u16>,
}

impl Path {
    #[allow(dead_code)]
    pub fn new<I, T>(endpoints: I, limit: u32, window_time: u16) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            paths: Some(endpoints.into_iter().map(Into::into).collect()),
            limit: Some(limit),
            window_time: Some(window_time),
        }
    }

    pub fn paths(&self) -> Option<Vec<String>> {
        self.paths.clone()
    }

    pub fn window_time(&self) -> Option<u16> {
        self.window_time
    }

    pub fn limit(&self) -> Option<u32> {
        self.limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_new() {
        let path = Path::new(vec!["test", "1234", "214141"], 300, 400);

        assert_eq!(path.paths.clone().unwrap()[0], "test".to_owned());
        assert_eq!(path.paths.clone().unwrap()[1], "1234".to_owned());
        assert_eq!(path.paths.clone().unwrap()[2], "214141".to_owned());
        assert_eq!(path.limit, Some(300));
        assert_eq!(path.window_time, Some(400));
    }
}
