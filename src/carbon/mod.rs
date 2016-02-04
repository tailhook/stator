use std::net::SocketAddr;

mod ffi;

pub use self::ffi::carbon_connect_ipv4;

pub type Seed = SocketAddr;
