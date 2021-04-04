use std::error::Error;

use crate::{Backend, Config};

pub struct Dur<T> {
    backend: T,
    config: Config,
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

    // pub fn increment(&mut self) {
    //     self.backend
    // }
}
