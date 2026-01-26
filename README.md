# Nord-Pproxy
`nord-proxy` is a Rust crate for retrieving NordVPN proxy endpoints (SOCKS5 and HTTP/HTTPS)
and integrating them with `reqwest`.


## Usage
```rs
use nord_proxy::{Proxy, Socks5, ProxyTrait};

// SOCKS5 proxies
let socks5 = Socks5::new().await;
let socks5_proxies = socks5.proxies("username", "password");

// HTTPS proxies
let proxy = Proxy::new().await;
let http_proxies = proxy.proxies("username", "password");

// Example: use with reqwest
let proxy_info = &http_proxies[0];
let client = reqwest::Client::builder()
    .proxy(reqwest::Proxy::all(proxy_info.proxy.clone())?)
    .build()?;
```
