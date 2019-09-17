# maxminddb-rust-bench

A quick benchmark to measure the performance of [maxminddb-rust](https://github.com/oschwald/maxminddb-rust).  

## Usage

First you need to have a working copy of the GeoIP City database.
You can fetch it from [here](https://dev.maxmind.com/geoip/geoip2/geolite2/).

Place it in this folder as `GeoIP2-City.mmdb`.

Once this is done, run

```
make bench
```

