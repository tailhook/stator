use std::os::unix::io::RawFd;
use std::mem::transmute;

use nix::sys::eventfd::{eventfd, EFD_CLOEXEC, EFD_NONBLOCK};
use nix::poll::{poll, PollFd, POLLIN, EventFlags};
use nix::unistd::{read, write};
use nix::Error;
use nix::Errno::EINTR;


#[derive(Debug)]
pub struct Async(RawFd);


impl Async {
    pub fn new() -> Async {
        Async(eventfd(0, EFD_CLOEXEC|EFD_NONBLOCK).expect("create eventfd"))
    }
    pub fn notify(&self) {
        let buf: [u8; 8] = unsafe { transmute(1u64) };
        write(self.0, &buf).expect("write eventfd");
    }
    pub fn check(&self) -> bool {
        let mut buf = [0u8; 8];
        loop {
            match read(self.0, &mut buf) {
                Ok(_) => return true,
                Err(Error::Sys(EINTR)) => continue,
                Err(_) => return false,
            }
        }
    }
    /// Wait (blocking) for notification, doesn't reset counter
    ///
    /// Returns `false` if wait was interrupted by some error
    /// (presumably EINTR)
    pub fn wait(&self) -> bool {
        let mut pollfd = [PollFd { fd: self.0, events: POLLIN,
            revents: EventFlags::empty() }];
        poll(&mut pollfd, -1).is_ok()
    }
    pub unsafe fn export_fd(&self) -> RawFd {
        self.0
    }
}
