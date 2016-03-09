use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::sync::atomic::AtomicUsize;
use std::collections::HashMap;

use rotor::{Loop, Config, Response};
use rotor_tools::loop_ext::LoopInstanceExt;

use super::{Context, Manager, Main, Fsm};

impl Manager {
    pub fn start() -> Manager {
        let creator = Loop::new(&Config::new()).unwrap();
        let mut inst = creator.instantiate(Context);
        let (queue, notifier) = inst.add_and_fetch(Fsm::Main, |scope| {
            let m = Main::new();
            let q = m.queue.clone();
            Response::ok((m, (q, scope.notifier())))
        }).unwrap();
        let (tx, rx) = channel();
        Manager {
            notifier: notifier,
            queue: queue,
            thread: thread::spawn(|| {
                inst.run()
            }),
            sockets: Arc::new(Mutex::new(HashMap::new())),
            id_gen: AtomicUsize::new(1),
            sender: Arc::new(Mutex::new(tx)),
            input: Arc::new(Mutex::new(rx)),
        }
    }
}
