use std::os::unix::io::RawFd;

use inner::MANAGER;


#[no_mangle]
pub extern fn stator_wait_message(
    cb: extern fn(u64, *const u8, usize) -> *mut u8,
    interrupted_cb: extern fn() -> *mut u8)
    -> *mut u8
{
    loop {
        {
            let mut inp = MANAGER.input.lock().expect("stator input");
            if let Some((sock_id, data)) = inp.pop_front() {
                return cb(sock_id as u64, &data[..][0], data.len())
            } else {
                MANAGER.input_notifier.check();
            }
        }
        if !MANAGER.input_notifier.wait() {
            // this allows KeyboardInterrupt
            return interrupted_cb();
        }
    }
}

#[no_mangle]
pub extern fn stator_next_message(
    cb: extern fn(u64, *const u8, usize) -> *mut u8,
    no_message_cb: extern fn() -> *mut u8)
    -> *mut u8
{
    let mut inp = MANAGER.input.lock().expect("stator input");
    if let Some((sock_id, data)) = inp.pop_front() {
        return cb(sock_id as u64, &data[..][0], data.len())
    } else {
        MANAGER.input_notifier.check();
        return no_message_cb();
    }
}

#[no_mangle]
pub unsafe extern fn stator_get_input_fd() -> RawFd
{
    MANAGER.input_notifier.export_fd()
}
