use std::{isize};
use std::sync::mpsc::channel;
use std::sync::atomic::Ordering;
use std::net::{SocketAddr, SocketAddrV4 as V4, Ipv4Addr};


use inner::{MANAGER, Command};
use inner::Socket::HttpServer;


#[no_mangle]
pub extern fn stator_http_bind_ipv4(ip: u32, port: u16) -> u64 {
    MANAGER.post_message(
        Command::NewHttp(SocketAddr::V4(V4::new(Ipv4Addr::from(ip), port))));
    let id = MANAGER.insert(HttpServer);
    return id as u64;
}
