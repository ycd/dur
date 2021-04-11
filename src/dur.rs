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
        match self.backend.insert(id, ip_and_path.clone()) {
            Ok(v) => {
                let current_timestamp = SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap();

                self.backend.evict_older_timestamps(
                    id,
                    current_timestamp,
                    self.config.window_time(),
                );

                let mut allow = true;

                if (v as u32) < self.config.limit() {
                    allow = false
                }

                match self.config.limited_ip_addresses() {
                    // TODO(ycd): properly test here.
                    Some(ip_addrs) => match ip_and_path.ip {
                        Some(ref ip) => {
                            if ip_addrs.contains(&ip.clone()) {
                                match self.config.ip_addresses_limit() {
                                    Some(limit) => {
                                        if limit as usize > self.backend.unique_ip_addresses(id) {
                                            allow = false
                                        }
                                    }
                                    None => (),
                                }
                            }
                        }
                        None => (),
                    },
                    None => (),
                }

                (allow, v)
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
