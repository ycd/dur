use std::time::SystemTime;

use crate::{Backend, Config, IpAndPath};

#[derive(Debug, Clone)]
pub struct Dur<T> {
    backend: T,
    pub config: Config,
}

impl<T> Dur<T>
where
    T: Backend,
{
    pub fn new(backend: T, config: Option<Config>) -> Self {
        Self {
            backend: backend,
            config: config.unwrap_or_default(),
        }
    }

    pub fn request(&mut self, id: u64, ip_and_path: IpAndPath) -> (bool, usize) {
        match self.backend.insert(id, ip_and_path) {
            Ok(v) => {
                let current_timestamp = SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap();

                self.backend.evict_older_timestamps(
                    id,
                    current_timestamp,
                    self.config.window_time(),
                );

                ((v as u32) < self.config.limit(), v)
            }

            // TODO: handle error better
            Err(why) => {
                eprintln!("an error occured: {}", why);
                (false, 0)
            }
        }
    }

    pub fn remaning_requests(&self, id: u64) -> u32 {
        self.backend.request_count(id) as u32
    }
}

#[cfg(test)]
mod tests {

    use std::thread::sleep;

    use super::*;
    use crate::Memory;

    #[test]
    fn test_sliding_window_logs() {
        let mut dur = Dur::new(Memory::new(), None);
        dur.config.set_window_time(1);

        dur.request(12938102, IpAndPath::new(None, None));
        dur.request(12938102, IpAndPath::new(None, None));
        dur.request(12938102, IpAndPath::new(None, None));
        dur.request(12938102, IpAndPath::new(None, None));
        dur.request(12938102, IpAndPath::new(None, None));
        dur.request(12938102, IpAndPath::new(None, None));

        assert_eq!(dur.backend.request_count(12938102), 6);
        sleep(std::time::Duration::from_secs(4));

        dur.request(12938102, IpAndPath::new(None, None));
        assert_eq!(dur.backend.request_count(12938102), 1);
    }
}
