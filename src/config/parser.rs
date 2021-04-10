use std::{fs::File, io::Read};

use crate::Config;

impl Config {
    pub fn from_path(path: String) -> Self {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();
        println!("{:#?}", config);
        config
    }
}
