pub use clap::{App, Arg};

use crate::Config;

static NAME: &str = "dur";
static VERSION: &str = env!("CARGO_PKG_VERSION");
static ABOUT: &str =
    "dur, lightweight, stateless, configurable rate limiter with extremely high-performance";

mod options {
    pub const CONFIG_PATH: &str = "config-path";
    pub const PORT: &str = "port";
    pub const HOST: &str = "host";
    pub const LIMIT: &str = "limit";
    pub const WINDOW_TIME: &str = "window-time";
    pub const IP_ADDR_LIMIT: &str = "ipaddr-limit";
}

pub fn cli() -> Config {
    let matches = App::new(NAME)
        .name(NAME)
        .version(VERSION)
        .about(ABOUT)
        .arg(
            Arg::with_name(options::CONFIG_PATH)
                .short("C")
                .long(options::CONFIG_PATH)
                .help("path to config file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name(options::PORT)
                .short("p")
                .long(options::PORT)
                .help("Bind socket to this port.")
                .default_value("8000")
                .takes_value(true)
        )
        .arg(
            Arg::with_name(options::HOST)
                .short("h")
                .long(options::HOST)
                .help("Bind socket to this host.")
                .default_value("127.0.0.1")
                .takes_value(true)
        )
        .arg(
            Arg::with_name(options::LIMIT)
                .short("L")
                .long(options::LIMIT)
                .help("The maximum number of requests to allow inside a window")
                .default_value("300")
                .takes_value(true),
            )        
        .arg(
            Arg::with_name(options::WINDOW_TIME)
                .short("WT")
                .long(options::WINDOW_TIME)
                .help("The window time, in seconds")
                .default_value("100")                
                .takes_value(true),
        )
        .arg(
            Arg::with_name(options::IP_ADDR_LIMIT)
                .short("IL")
                .long(options::IP_ADDR_LIMIT)
                .help("The maximum number of requests to allow from unique ip addresses inside a window")
                .default_value("5")
                .takes_value(true),
        )
        .get_matches();

    let limit = matches.value_of(options::LIMIT).unwrap().parse::<u32>().unwrap();
    let ip_addr_limit = matches.value_of(options::IP_ADDR_LIMIT).unwrap().parse::<u16>().unwrap();
    let window_time =matches.value_of(options::WINDOW_TIME).unwrap().parse::<u16>().unwrap();
    let port = matches.value_of(options::PORT).unwrap().to_owned();
    let host = matches.value_of(options::HOST).unwrap().to_owned();

    match matches.value_of(options::CONFIG_PATH) {
        Some(path) => Config::from_path(path.to_owned()),
        None => Config::new(Some(limit), Some(ip_addr_limit), Some(window_time), Some(port), Some(host))
    }   
}
