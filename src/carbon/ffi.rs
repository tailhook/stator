use std::net::{SocketAddr, SocketAddrV4 as V4, Ipv4Addr};
use std::{slice, str};
use std::sync::mpsc::channel;

use inner::{MANAGER, Command, SockId};
use inner::Socket::Carbon;

#[no_mangle]
pub extern fn stator_carbon_connect_ipv4(ip: u32, port: u16) -> u64 {
    let (tx, rx) = channel();
    MANAGER.post_message(
        Command::NewCarbon((
            SocketAddr::V4(V4::new(Ipv4Addr::from(ip), port)),
            tx,
        )));
    let sink = rx.recv().expect("carbon sink received");
    return MANAGER.insert(Carbon(sink)) as u64;
}

#[no_mangle]
pub unsafe extern fn stator_carbon_add_i64(socket: u64,
    name: *const u8, name_len:usize, value: i64)
{
    let slice = slice::from_raw_parts(name, name_len);
    let name = str::from_utf8(slice).unwrap();
    MANAGER.with_socket(socket as SockId, |sock| {
        match *sock {
            Carbon(ref sink) => sink.sender().add_value(name, value),
            _ => {}
        }
    }).ok();
}

#[no_mangle]
pub unsafe extern fn stator_carbon_add_i64_at(socket: u64,
    name: *const u8, name_len:usize, value: i64, timestamp: u64)
{
    let slice = slice::from_raw_parts(name, name_len);
    let name = str::from_utf8(slice).unwrap();
    MANAGER.with_socket(socket as SockId, |sock| {
        match *sock {
            Carbon(ref sink) => {
                sink.sender().add_value_at(name, value, timestamp)
            }
            _ => {}
        }
    }).ok();
}

#[no_mangle]
pub unsafe extern fn stator_carbon_add_f64(socket: u64,
    name: *const u8, name_len:usize, value: f64)
{
    let slice = slice::from_raw_parts(name, name_len);
    let name = str::from_utf8(slice).unwrap();
    MANAGER.with_socket(socket as SockId, |sock| {
        match *sock {
            Carbon(ref sink) => sink.sender().add_value(name, value),
            _ => {}
        }
    }).ok();
}

#[no_mangle]
pub unsafe extern fn stator_carbon_add_f64_at(socket: u64,
    name: *const u8, name_len:usize, value: f64, timestamp: u64)
{
    let slice = slice::from_raw_parts(name, name_len);
    let name = str::from_utf8(slice).unwrap();
    MANAGER.with_socket(socket as SockId, |sock| {
        match *sock {
            Carbon(ref sink) => {
                sink.sender().add_value_at(name, value, timestamp)
            }
            _ => {}
        }
    }).ok();
}
