#[macro_use]
extern crate criterion;
extern crate maxminddb_rust_bench;

use rayon::prelude::*;

use criterion::black_box;
use criterion::Criterion;

use maxminddb::geoip2;
use maxminddb_rust_bench::generate_ipv4;
use std::net::IpAddr;

// Single-threaded
pub fn bench_maxminddb(ips: &Vec<IpAddr>, reader: &maxminddb::Reader<Vec<u8>>) {
    ips.iter().for_each(|ip| {
        let city: Result<geoip2::City, _> = reader.lookup(*ip);
    });
}

// Using rayon for parallel execution
pub fn bench_par_maxminddb(ips: &Vec<IpAddr>, reader: &maxminddb::Reader<Vec<u8>>) {
    ips.par_iter().for_each(|ip| {
        let city: Result<geoip2::City, _> = reader.lookup(*ip);
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let ips = generate_ipv4(10_000);
    let reader = maxminddb::Reader::open_readfile("GeoIP2-City.mmdb").unwrap();

    c.bench_function("maxminddb", |b| b.iter(|| bench_maxminddb(&ips, &reader)));
}

pub fn criterion_par_benchmark(c: &mut Criterion) {
    let ips = generate_ipv4(10_000);
    let reader = maxminddb::Reader::open_readfile("GeoIP2-City.mmdb").unwrap();

    c.bench_function("maxminddb_par", |b| {
        b.iter(|| bench_par_maxminddb(&ips, &reader))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10);

    targets = criterion_benchmark, criterion_par_benchmark
}
criterion_main!(benches);
