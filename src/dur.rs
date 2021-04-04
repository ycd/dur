use crate::{Backend, Config};

pub struct Dur {
    backend: Backend,
    config: Config,
}

impl Dur {
    pub fn new(backend: Backend, config: Option<Config>) -> Self {
        Self {
            backend: Backend::Memory,
            config: config.unwrap_or_default(),
        }
    }
}

impl Default for Dur {
    fn default() -> Self {
        Self {
            backend: Backend::Memory,
            config: Config::default(),
        }
    }
}
