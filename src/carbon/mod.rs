use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::sync::atomic::AtomicIsize;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::collections::HashMap;

use rotor::Scope;
use rotor_carbon::{Sink, connect_ip};

pub mod ffi;

pub use rotor_carbon::Fsm;
pub type Seed = (SocketAddr, Sender<Sink>);

pub struct Holder {
    counter: AtomicIsize,
    sinks: Arc<Mutex<HashMap<isize, Sink>>>,
}

impl Holder {
    pub fn new() -> Holder {
        Holder {
            counter: AtomicIsize::new(1),
            sinks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub fn create<C>((addr, sender): Seed, scope: &mut Scope<C>)
    -> Result<Fsm<C>, Box<Error>>
{
    let (fsm, sink) = try!(connect_ip(addr, scope));
    sender.send(sink).unwrap();
    Ok(fsm)
}
