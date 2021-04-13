use std::{net::Ipv4Addr, str::FromStr};

pub use clap::{App, Arg};

use crate::{
    config::{Ip, Limits, Path},
    Config,
};

static NAME: &str = "dur";
static VERSION: &str = env!("CARGO_PKG_VERSION");
static ABOUT: &str =
    "dur, lightweight, stateless, configurable rate limiter with extremely high-performance";

#[allow(dead_code)]
mod options {
    pub const CONFIG_PATH: &str = "config-path";
    pub const PORT: &str = "port";
    pub const HOST: &str = "host";
    pub const LIMIT: &str = "limit";
    pub const WINDOW_TIME: &str = "window-time";
    pub const IP_ADDR_LIMIT: &str = "ipaddr-limit";
    pub const PATHS: &str = "paths";
    pub const PATH_LIMIT: &str = "path-limit";
    pub const PATH_WINDOW_TIME: &str = "path-window-time";
    pub const IP_ADDRESSES: &str = "ip-addresses";
    pub const IP_ADDRESSES_LIMIT: &str = "ip-addresses-limit";
    pub const IP_ADDRESSES_WINDOW_TIME: &str = "ip-addresses-window-time";
}

pub fn cli() -> Config {
    let matches = App::new(NAME)
        .name(NAME)
        .version(VERSION)
        .about(ABOUT)
        .arg(
            Arg::with_name(options::CONFIG_PATH)
                .short("c")
                .long(options::CONFIG_PATH)
                .help("path to config file")
                .takes_value(true)
                .value_name("PATH"),
        )
        .arg(
            Arg::with_name(options::PORT)
                .short("p")
                .long(options::PORT)
                .help("Bind socket to this port.")
                .default_value("8000")
                .takes_value(true)
                .value_name("PORT"),
        )
        .arg(
            Arg::with_name(options::HOST)
                .short("h")
                .long(options::HOST)
                .help("Bind socket to this host.")
                .default_value("127.0.0.1")
                .takes_value(true)
                .value_name("HOST"),
        )
        .arg(
            Arg::with_name(options::LIMIT)
                .short("L")
                .long(options::LIMIT)
                .help("The maximum number of requests to allow inside a window")
                .default_value("300")
                .value_name("INT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(options::WINDOW_TIME)
                .long(options::WINDOW_TIME)
                .help("The window time, in seconds")
                .default_value("100")
                .value_name("INT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(options::IP_ADDR_LIMIT)
                .long(options::IP_ADDR_LIMIT)
                .help("The maximum number of requests to allow from specified ip addresses")
                .default_value("5")
                .value_name("INT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(options::PATHS)
                .short("P")
                .long(options::PATHS)
                .help("Paths to be specifically limited, with comma seperated values")
                .value_name("PATH,PATH...")
                .takes_value(true)
                .require_delimiter(true),
        )
        .arg(
            Arg::with_name(options::PATH_LIMIT)
                .long(options::PATH_LIMIT)
                .help("The maximum number of requests to allow in specified paths")
                .requires(options::PATHS)
                .value_name("INT")
                .takes_value(true)
                .default_value_if(options::PATHS, None, "300"),
        )
        .arg(
            Arg::with_name(options::PATH_WINDOW_TIME)
                .long(options::PATH_WINDOW_TIME)
                .help("The window time for paths, in seconds")
                .requires(options::PATHS)
                .value_name("INT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(options::IP_ADDRESSES)
                .short("I")
                .long(options::IP_ADDRESSES)
                .help("IP Addresses to be specifically limited, with comma seperated values")
                .value_name("IP,IP...")
                .takes_value(true)
                .require_delimiter(true),
        )
        .arg(
            Arg::with_name(options::IP_ADDRESSES_LIMIT)
                .long(options::IP_ADDRESSES_LIMIT)
                .help("The maximum number of requests to allow in specified IP addresses")
                .requires(options::PATHS)
                .value_name("INT")
                .takes_value(true)
                .default_value_if(options::IP_ADDRESSES, None, "300"),
        )
        .arg(
            Arg::with_name(options::IP_ADDRESSES_WINDOW_TIME)
                .long(options::IP_ADDRESSES_WINDOW_TIME)
                .help("The window time for IP addresses, in seconds")
                .requires(options::IP_ADDRESSES)
                .value_name("INT")
                .takes_value(true),
        )
        .get_matches();

    let limit = matches
        .value_of(options::LIMIT)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let ip_addr_limit = matches
        .value_of(options::IP_ADDR_LIMIT)
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let window_time = matches
        .value_of(options::WINDOW_TIME)
        .unwrap()
        .parse::<u16>()
        .unwrap();

    let limits: Limits = {
        let path: Option<Path> = match matches.values_of(options::PATHS) {
            Some(v) => {
                let vals: Vec<String> = v.map(String::from).collect();
                let path_window_time = matches
                    .value_of(options::PATH_WINDOW_TIME)
                    .unwrap_or("300")
                    .parse::<u16>()
                    .unwrap();

                let path_limit = matches
                    .value_of(options::PATH_LIMIT)
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap();

                Some(Path::new(vals, path_limit, path_window_time))
            }
            None => None,
        };

        let ip: Option<Ip> = match matches.values_of(options::IP_ADDRESSES) {
            Some(v) => {
                let vals: Vec<Ipv4Addr> = v
                    .map(|s| match Ipv4Addr::from_str(s) {
                        Ok(addr) => addr,
                        Err(e) => panic!("bad ip address: {}", e),
                    })
                    .collect();
                let ip_window_time = matches
                    .value_of(options::IP_ADDRESSES_WINDOW_TIME)
                    .unwrap_or("300")
                    .parse::<u16>()
                    .unwrap();

                let ip_limit = matches
                    .value_of(options::IP_ADDRESSES_LIMIT)
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap();

                Some(Ip::new(vals, ip_limit, ip_window_time))
            }
            None => None,
        };

        Limits::new(path, ip)
    };

    let port = matches.value_of(options::PORT).unwrap().to_owned();
    let host = matches.value_of(options::HOST).unwrap().to_owned();

    match matches.value_of(options::CONFIG_PATH) {
        Some(path) => Config::from_path(path.to_owned()),
        None => Config::new(
            Some(limit),
            Some(ip_addr_limit),
            Some(window_time),
            Some(port),
            Some(host),
            limits,
        ),
    }
}
