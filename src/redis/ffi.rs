use std::net::{SocketAddr, SocketAddrV4 as V4, Ipv4Addr};
use std::sync::mpsc::channel;

use inner::{MANAGER, Command, SockId};

#[repr(C)]
pub struct Arg {
    data: *const u8,
    len: usize,
}

pub struct ArgSet {
    items: *const Arg,
    num: usize,
}

#[no_mangle]
pub extern fn stator_redis_connect_ipv4(ip: u32, port: u16, db: u32) -> u64 {
    let (tx, rx) = channel();
    MANAGER.post_message(
        Command::NewRedis((
            SocketAddr::V4(V4::new(Ipv4Addr::from(ip), port)),
            db,
            tx,
        )));
    return rx.recv().expect("redis client socket id received") as u64;
}

#[no_mangle]
pub extern fn stator_redis_queue(sock: u64, args: *const Arg, num: usize)
    -> u64
{
    unimplemented!();
}
