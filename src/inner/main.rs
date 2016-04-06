use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use rotor::void::{Void, unreachable};
use rotor::{Machine, Scope, EventSet, Response};
use rotor::mio::tcp::TcpStream;

use super::{Context, Fsm, Main, Command, Command as C};
use carbon;
use http;
use redis;

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
    type Seed = Command;
    fn create(seed: Command, scope: &mut Scope<Self::Context>)
        -> Response<Self, Void>
    {
        match seed {
            Command::NewCarbon(x) => {
                carbon::create(x, scope).wrap(Fsm::Carbon)
            }
            Command::NewRedis(x) => {
                redis::create(x, scope).wrap(Fsm::Redis)
            }
            Command::NewHttp(x) => {
                http::create(x, scope).wrap(Fsm::Http)
            }
            Command::AcceptHttp(x) => {
                <http::Fsm as Machine>::create(x, scope).wrap(Fsm::Http)
            }
        }
    }

    fn ready(self, ev: EventSet, scope: &mut Scope<Self::Context>)
        -> Response<Self, Self::Seed>
    {
        match self {
            Fsm::Main(_) => Response::ok(self),
            Fsm::Carbon(x) => x.ready(ev, scope).map(Fsm::Carbon, void),
            Fsm::Http(x) => x.ready(ev, scope).map(Fsm::Http, C::AcceptHttp),
            Fsm::Redis(x) => x.ready(ev, scope).map(Fsm::Redis, void),
        }
    }
    fn spawned(self, scope: &mut Scope<Self::Context>)
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
            Fsm::Carbon(x) => x.spawned(scope).map(Fsm::Carbon, void),
            Fsm::Redis(x) => x.spawned(scope).map(Fsm::Redis, void),
            Fsm::Http(x) => x.spawned(scope).map(Fsm::Http, C::AcceptHttp),
        }
    }
    fn timeout(self, scope: &mut Scope<Self::Context>)
        -> Response<Self, Self::Seed>
    {
        match self {
            Fsm::Main(_) => Response::ok(self),
            Fsm::Carbon(x) => x.timeout(scope).map(Fsm::Carbon, void),
            Fsm::Redis(x) => x.timeout(scope).map(Fsm::Redis, void),
            Fsm::Http(x) => x.timeout(scope).map(Fsm::Http, C::AcceptHttp),
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
            Fsm::Redis(x) => x.wakeup(scope).map(Fsm::Redis, void),
            Fsm::Http(x) => x.wakeup(scope).map(Fsm::Http, C::AcceptHttp),
        }
    }
}
