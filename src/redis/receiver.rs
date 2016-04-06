use rotor_redis;
use rotor_redis::Message::{self};

use inner::SockId;

#[derive(Clone)]
pub struct Receiver(SockId);

impl Receiver {
    pub fn new(id: SockId) -> Receiver {
        Receiver(id)
    }
}

impl rotor_redis::Receiver for Receiver {
    fn receive(&mut self, msg: &Message) {
        unimplemented!();
    }
}
