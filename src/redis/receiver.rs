use std::io::Cursor;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

use rotor_redis;
use rotor_redis::Message;
use cbor::Encoder;

use inner::{MANAGER, SockId};

pub struct Receiver(SockId, usize, Arc<AtomicUsize>);

impl Receiver {
    pub fn new(id: SockId) -> Receiver {
        Receiver(id, 0, Arc::new(AtomicUsize::new(0)))
    }
}

impl Clone for Receiver {
    fn clone(&self) -> Receiver {
        Receiver(self.0, self.2.load(SeqCst), self.2.clone())
    }
}

fn write_message(enc: &mut Encoder<Cursor<Vec<u8>>>, msg: &Message) {
    use rotor_redis::Message::*;
    match *msg {
        Simple(s) => enc.text(s).unwrap(),
        Error(kind, text) => {
            enc.object(2).unwrap();
            enc.text("error_kind").unwrap();
            enc.text(kind).unwrap();
            enc.text("error_text").unwrap();
            enc.text(text).unwrap();
        }
        Int(x) => enc.i64(x).unwrap(),
        Bytes(x) => enc.bytes(x).unwrap(),
        Null => enc.null().unwrap(),
        Array(_) => unimplemented!(),
    }
}

impl Receiver {
    pub fn next_id(&mut self) -> usize {
        self.2.fetch_add(1, SeqCst)
    }
}

impl rotor_redis::Receiver for Receiver {
    fn receive(&mut self, msg: &Message) {
        let mut enc = Encoder::new(Cursor::new(Vec::new()));
        write_message(&mut enc, msg);
        let vec = enc.into_writer().into_inner();
        MANAGER.send(self.0, vec.into_boxed_slice());
        self.1 += 1;
    }
}
