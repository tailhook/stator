use std::net::{SocketAddr, SocketAddrV4 as V4, Ipv4Addr};
use std::sync::mpsc::channel;

use rotor_redis::conversion::ToRedisCommand;
use rotor_stream::Buf;

use inner::{MANAGER, Command, SockId, Socket};

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
    MANAGER.with_socket(sock as SockId, |s| {
        match s {
            &mut Socket::Redis(ref red) => {
                let mut lock = red.0.lock().expect("lock redis connection");
                ArgSet { items: args, num: num }.write_into(
                    lock.transport().expect("valid redis transport").output());
                let id = lock.protocol().expect("valid redis proto")
                    .receiver.next_id();
                red.1.wakeup().expect("redis notify");
                return id;
            }
            _ => {
                error!("Socket {} is a redis", sock);
                return 0;
            }
        }
    }).unwrap_or(0) as u64
}

impl ToRedisCommand for ArgSet {
    fn write_into(self, buf: &mut Buf) {
    }
}
