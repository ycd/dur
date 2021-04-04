use std::net::IpAddr;

use crate::{Backend, Config};

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

    pub fn request(&mut self, id: i64, ip_addr: Option<IpAddr>) -> bool {
        match self.backend.insert(id, ip_addr) {
            Ok(v) => {
                if v as u32 > self.config.limit() {
                    return false;
                } else {
                    return true;
                }
            }

            // TODO: handle error better
            Err(why) => {
                eprintln!("an error occured: {}", why);
                false
            }
        }
    }
}
