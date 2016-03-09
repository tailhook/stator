use inner::MANAGER;


#[no_mangle]
pub extern fn stator_wait_message(
    cb: extern fn(u64, *const u8, usize) -> *mut u8)
    -> *mut u8
{
    let (sock_id, data) = MANAGER.input.lock().expect("stator_input")
                            .recv().expect("something received");
    return cb(sock_id as u64, &data[..][0], data.len())
}
