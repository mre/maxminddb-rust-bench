use fake::faker::internet::raw::*;

use fake::locales::EN;
use fake::Fake;
use std::net::IpAddr;
use std::str::FromStr;

// Generate `count` IPv4 addresses
pub fn generate_ipv4(count: u64) -> Vec<IpAddr> {
    let mut ips = Vec::new();
    for _i in 0..count {
        let val: String = IPv4(EN).fake();
        let ip: IpAddr = FromStr::from_str(&val).unwrap();
        ips.push(ip);
    }
    ips
}
