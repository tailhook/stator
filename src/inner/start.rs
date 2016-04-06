use std::thread;
use std::env;
use std::sync::{Arc, Mutex,};
use std::sync::mpsc::sync_channel;
use std::sync::atomic::AtomicUsize;
use std::collections::{HashMap, VecDeque};

use env_logger;
use rotor::{Loop, Config, Response};
use rotor_tools::loop_ext::LoopInstanceExt;

use super::{Context, Manager, Main, Fsm};
use super::eventfd::Async;

impl Manager {
    pub fn start() -> Manager {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "warn");
        }
        env_logger::init().expect("init rust logging");
        let (tx, rx) = sync_channel(1);
        let thread = thread::spawn(|| {
            let creator = Loop::new(&Config::new())
                          .expect("create rotor loop");
            let mut inst = creator.instantiate(Context);
            let (queue, notifier) = inst.add_and_fetch(Fsm::Main, |scope| {
                let m = Main::new();
                let q = m.queue.clone();
                Response::ok((m, (q, scope.notifier())))
            }).unwrap();
            {tx}.send((queue, notifier));
            inst.run()
        });
        let (queue, notifier) = {rx}.recv().expect("message from I/O thread");
        Manager {
            notifier: notifier,
            queue: queue,
            thread: thread,
            sockets: Arc::new(Mutex::new(HashMap::new())),
            id_gen: AtomicUsize::new(1),
            input: Arc::new(Mutex::new(VecDeque::new())),
            input_notifier: Async::new(),
        }
    }
}
