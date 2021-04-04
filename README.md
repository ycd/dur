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
    * Path based limiting 
    * IP based limiting
    * Service based limiting


## Examples

   TODO: Add config example.

---

USAGE SHOULD BE HERE

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
