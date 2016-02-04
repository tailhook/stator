use std::error::Error;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use rotor::{Machine, Scope, EventSet, Response};

use super::{Context, Fsm, Main, Seed};

impl Main {
    pub fn new() -> Main {
        Main {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

impl Machine for Fsm {
    type Context = Context;
    type Seed = Seed;
    fn create(_seed: Self::Seed, _scope: &mut Scope<Self::Context>)
        -> Result<Self, Box<Error>>
    {
        unimplemented!();
    }

    fn ready(self, _events: EventSet, _scope: &mut Scope<Self::Context>)
        -> Response<Self, Self::Seed>
    {
        unimplemented!();
    }
    fn spawned(self, _scope: &mut Scope<Self::Context>)
        -> Response<Self, Self::Seed>
    {
        unimplemented!();
    }
    fn timeout(self, _scope: &mut Scope<Self::Context>)
        -> Response<Self, Self::Seed>
    {
        Response::ok(self)
    }
    fn wakeup(self, _scope: &mut Scope<Self::Context>)
        -> Response<Self, Self::Seed>
    {
        match self {
            Fsm::Main(main) => {
                let maybe_el = main.queue.lock().unwrap().pop_front();
                if let Some(el) = maybe_el {
                    Response::spawn(Fsm::Main(main), el)
                } else {
                    Response::ok(Fsm::Main(main))
                }
            }
        }
    }
}
