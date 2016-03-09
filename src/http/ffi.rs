use std::net::{SocketAddr, SocketAddrV4 as V4, Ipv4Addr};


use inner::{MANAGER, Command};
use inner::Socket::HttpServer;


#[no_mangle]
pub extern fn stator_http_bind_ipv4(ip: u32, port: u16) -> u64 {
    let id = MANAGER.insert(HttpServer);
    MANAGER.post_message(Command::NewHttp((
        SocketAddr::V4(V4::new(Ipv4Addr::from(ip), port)),
        id,
    )));
    return id as u64;
}
