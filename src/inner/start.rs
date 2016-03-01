use std::thread;

use rotor::{Loop, Config, Response};
use rotor_tools::loop_ext::LoopInstanceExt;

use super::{Context, Manager, Main, Fsm};
use carbon;

impl Manager {
    pub fn start() -> Manager {
        let creator = Loop::new(&Config::new()).unwrap();
        let mut inst = creator.instantiate(Context);
        let (queue, notifier) = inst.add_and_fetch(Fsm::Main, |scope| {
            let m = Main::new();
            let q = m.queue.clone();
            Response::ok((m, (q, scope.notifier())))
        }).unwrap();
        Manager {
            notifier: notifier,
            queue: queue,
            thread: thread::spawn(|| {
                inst.run()
            }),
            carbon: carbon::Holder::new(),
        }
    }
}
