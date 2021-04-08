<div align="center">

![](assets/dur.png)

[dur](https://github.com/ycd/dur) lightweight, stateless, configurable rate limiter with extremely high-performance.   




</div>



---

## Features

* Dur can be used a sidecar to services that needs independent rate-limiting.
* Dur provides a very modern access to its API via HTTP.
* Can work as stateless which means dur doesn't do any read/write on disk. 
* Dur can work with a centralized caching systems such as Redis. 
* Extremely high performance thanks to Rust, dur handles +20K request/s with 500 concurrency on a single core with 0.05s response time on average.
* Provides highly configurable limiting
    * Path based limiting [TODO]
    * IP based limiting 
    * Service based limiting


## Rate Limiting Algorithm

Dur uses **sliding window log** algorithm for rate limiting.

Sliding window log algorithm keeps a log of request timestamps for each user. When a request comes, dur first pop all outdated timestamps before appending the new request time to the log. Then dur decides whether the request should be processed depending on whether the log size has exceeded the limit.

## Example Configuration File

```toml
[config]

# the limit inside a window
limit = 300

# Window time in seconds.
window_time = 300

# The maximum number of unique ip addresses for one user inside a window
ip_addr_limit = 5

# Explicitly rate limiting some endpoints.
[limits.path]

paths = [
  "/abc/def/gef/asdf",
  "/explicitly/limiting",
]


limit = 20

# Explicitly rate limiting specific IP addresses.
[limits.ip]

ip_addresses = [
  "10.27.104.11",
  "10.27.104.12",
]

limit = 100

window_time = 50
```

---

## Usage


```bash
dur 0.1.0
dur, lightweight, stateless, configurable rate limiter with extremely high-performance

USAGE:
    dur [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -C, --config-path <config-path>      path to config file
    -h, --host <host>                    Bind socket to this host. [default: 127.0.0.1]
    -I, --ipaddr-limit <ipaddr-limit>    The maximum number of requests to allow from unique ip addresses inside a
                                         window [default: 5]
    -L, --limit <limit>                  The maximum number of requests to allow inside a window [default: 300]
    -p, --port <port>                    Bind socket to this port. [default: 8000]
    -W, --window-time <window-time>      The window time, in seconds [default: 100]
```


---

## Installation

TODO: Provide a docker image



## Documentation

## API


### Health Check


#### Request

```
GET /health
```

#### Response

```json
{
    "status": "ok"
}
```

### Produce new requests

#### Request

```
GET /request/<id>
```

#### Response 

```json
{
    "allow": true
}
```

## Licence

dur’s source code is licenced under the [MIT License](https://www.mit.edu/~amini/LICENSE.md).
