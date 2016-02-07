use std::error::Error;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use void::{Void, unreachable};
use rotor::{Machine, Scope, EventSet, Response};

use super::{Context, Fsm, Main, Seed};
use carbon;

impl Main {
    pub fn new() -> Main {
        Main {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

fn void<T>(x: Void) -> T {
    unreachable(x);
}

impl Machine for Fsm {
    type Context = Context;
    type Seed = Seed;
    fn create(seed: Self::Seed, scope: &mut Scope<Self::Context>)
        -> Result<Self, Box<Error>>
    {
        match seed {
            Seed::Carbon(x) => carbon::create(x, scope).map(Fsm::Carbon),
        }
    }

    fn ready(self, ev: EventSet, scope: &mut Scope<Self::Context>)
        -> Response<Self, Self::Seed>
    {
        match self {
            Fsm::Main(_) => Response::ok(self),
            Fsm::Carbon(x) => x.ready(ev, scope).map(Fsm::Carbon, void),
        }
    }
    fn spawned(self, scope: &mut Scope<Self::Context>)
        -> Response<Self, Self::Seed>
    {
        match self {
            Fsm::Main(_) => Response::ok(self),
            Fsm::Carbon(x) => x.spawned(scope).map(Fsm::Carbon, void),
        }
    }
    fn timeout(self, scope: &mut Scope<Self::Context>)
        -> Response<Self, Self::Seed>
    {
        match self {
            Fsm::Main(_) => Response::ok(self),
            Fsm::Carbon(x) => x.timeout(scope).map(Fsm::Carbon, void),
        }
    }
    fn wakeup(self, scope: &mut Scope<Self::Context>)
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
            Fsm::Carbon(x) => x.wakeup(scope).map(Fsm::Carbon, void),
        }
    }
}
