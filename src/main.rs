use fake::faker::internet::raw::*;
use rayon::prelude::*;

use fake::locales::EN;
use fake::Fake;
use maxminddb::geoip2;
use std::net::IpAddr;
use std::str::FromStr;

// Generate `count` IPv4 addresses
fn generate_ipv4(count: u64) -> Vec<IpAddr> {
    let mut ips = Vec::new();
    for _i in 0..count {
        let val: String = IPv4(EN).fake();
        let ip: IpAddr = FromStr::from_str(&val).unwrap();
        ips.push(ip);
    }
    ips
}

fn main() {
    let reader = maxminddb::Reader::open_readfile("GeoIP2-City.mmdb").unwrap();

    println!("Generating fake Ips");
    let ips = generate_ipv4(1_000_000);

    println!("Lookup");
    ips.iter().for_each(|ip| {
        let city: Result<geoip2::City, _> = reader.lookup(*ip);
        // dbg!(city);
    });
}
