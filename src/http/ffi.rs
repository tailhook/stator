use std::slice;
use std::net::{SocketAddr, SocketAddrV4 as V4, Ipv4Addr};


use inner::{MANAGER, Command, SockId};
use inner::Socket::{HttpServer, HttpRequest};


#[no_mangle]
pub extern fn stator_http_bind_ipv4(ip: u32, port: u16) -> u64 {
    let id = MANAGER.insert(HttpServer);
    MANAGER.post_message(Command::NewHttp((
        SocketAddr::V4(V4::new(Ipv4Addr::from(ip), port)),
        id,
    )));
    return id as u64;
}

#[no_mangle]
pub extern fn stator_http_reply(sock: u64, data: *const u8, data_len: usize) {
    MANAGER.with_socket(sock as SockId, |s| {
        match s {
            &mut HttpRequest(ref mut place, ref mut notifier) => {
                let mut vec = Vec::with_capacity(data_len);
                vec.extend_from_slice(unsafe {
                    slice::from_raw_parts(data, data_len)
                });
                let mut ptr = place.lock().expect("lock http request mutex");
                *ptr = Some(vec.into_boxed_slice());
                notifier.wakeup().expect("notify succeds");
            }
            _ => {
                error!("Socket {} is not a http request", sock);
            }
        }
    }).map_err(|()| info!("Socket {} is not found", sock)).ok();
}
