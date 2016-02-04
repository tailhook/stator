use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::sync::atomic::AtomicIsize;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use rotor_carbon::Sink;

pub mod ffi;

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
