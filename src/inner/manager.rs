use std::usize;
use std::sync::atomic::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use super::{Manager, Command, Socket, SockId};


impl Manager {
    pub fn post_message(&self, msg: Command) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(msg);
        if q.len() == 1 {
            self.notifier.wakeup().unwrap();
        }
    }
    pub fn send(&self, sock_id: SockId, buf: Box<[u8]>) {
        self.sender.lock()
        .expect("sender is not poisoned")
        .send((sock_id, buf))
        .expect("send succeeds")
    }
    pub fn insert(&self, mut sock: Socket) -> SockId {
        loop {
            let nid = self.id_gen.fetch_add(1, Ordering::SeqCst);
            if nid == usize::MAX {
                self.id_gen.store(1, Ordering::SeqCst);
                continue;
            }
            let mut st = self.sockets.lock().expect("stator sockets lock");
            match st.entry(nid) {
                Occupied(..) => continue,
                Vacant(x) => {
                    x.insert(sock);
                    return nid;
                }
            }
        }
    }
    pub fn with_socket<T, F>(&self, id: SockId, fun: F) -> Result<T, ()>
        where F: FnOnce(&mut Socket) -> T
    {
        let mut st = self.sockets.lock().expect("stator sockets lock");
        st.get_mut(&id).map(fun).ok_or(())
    }
}
