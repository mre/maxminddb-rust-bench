# maxminddb-rust-bench

A quick benchmark to measure the performance of [maxminddb-rust].  

## Usage

First you need to have a working copy of the GeoIP City database.  
You can fetch it from [here](https://dev.maxmind.com/geoip/geoip2/geolite2/).
Place it in this folder as `GeoIP2-City.mmdb`.

Once this is done, run

```
cargo bench
```

This will look up 10_000 random IPs and measure the runtime with 
[criterion.rs].

It will also run a benchmark using [rayon]
for additional performance.

[rayon]: https://github.com/rayon-rs/rayon
[maxminddb-rust]: https://github.com/oschwald/maxminddb-rust
[criterion.rs]: https://github.com/bheisler/criterion.rs