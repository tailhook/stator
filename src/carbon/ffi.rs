use std::net::{SocketAddr, SocketAddrV4 as V4, Ipv4Addr};
use std::{isize, slice, str};
use std::sync::mpsc::channel;
use std::sync::atomic::Ordering;

use inner::{MANAGER, Seed};

#[no_mangle]
pub extern fn carbon_connect_ipv4(ip: u32, port: u16) -> isize {
    let (tx, rx) = channel();
    MANAGER.add_machine(
        Seed::Carbon((
            SocketAddr::V4(V4::new(Ipv4Addr::from(ip), port)),
            tx,
        )));
    // TODO(tailhook) generate id
    let sink = rx.recv().unwrap();
    let id = MANAGER.carbon.counter.fetch_add(1, Ordering::Relaxed);
    assert!(id < isize::MAX);
    let ref mut sinks = MANAGER.carbon.sinks.lock().unwrap();
    sinks.insert(id, sink);
    return id;
}

#[no_mangle]
pub unsafe extern fn carbon_add_i64(port: isize,
    name: *const u8, name_len:usize, value: i64)
{
    let slice = slice::from_raw_parts(name, name_len);
    let name = str::from_utf8(slice).unwrap();
    MANAGER.carbon.sinks.lock().map(|sinks| {
        sinks.get(&port).map(|sink| {
            sink.sender().add_value(name, value)
        });
    }).ok();
}

#[no_mangle]
pub unsafe extern fn carbon_add_i64_at(port: isize,
    name: *const u8, name_len:usize, value: i64, timestamp: u64)
{
    let slice = slice::from_raw_parts(name, name_len);
    let name = str::from_utf8(slice).unwrap();
    MANAGER.carbon.sinks.lock().map(|sinks| {
        sinks.get(&port).map(|sink| {
            sink.sender().add_value_at(name, value, timestamp)
        });
    }).ok();
}

#[no_mangle]
pub unsafe extern fn carbon_add_f64(port: isize,
    name: *const u8, name_len:usize, value: f64)
{
    let slice = slice::from_raw_parts(name, name_len);
    let name = str::from_utf8(slice).unwrap();
    MANAGER.carbon.sinks.lock().map(|sinks| {
        sinks.get(&port).map(|sink| {
            sink.sender().add_value(name, value)
        });
    }).ok();
}

#[no_mangle]
pub unsafe extern fn carbon_add_f64_at(port: isize,
    name: *const u8, name_len:usize, value: f64, timestamp: u64)
{
    let slice = slice::from_raw_parts(name, name_len);
    let name = str::from_utf8(slice).unwrap();
    MANAGER.carbon.sinks.lock().map(|sinks| {
        sinks.get(&port).map(|sink| {
            sink.sender().add_value_at(name, value, timestamp)
        });
    }).ok();
}
