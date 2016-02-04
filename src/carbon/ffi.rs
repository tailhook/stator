use std::net::{SocketAddr, SocketAddrV4 as V4, Ipv4Addr};

use inner::{MANAGER, Seed};

#[no_mangle]
pub extern fn carbon_connect_ipv4(ip: u32, port: u16) -> isize {
    MANAGER.add_machine(
        Seed::Carbon(SocketAddr::V4(V4::new(Ipv4Addr::from(ip), port))));
    // TODO(tailhook) generate id
    0
}
