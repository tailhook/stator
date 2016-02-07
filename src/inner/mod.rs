mod start;
mod main;
mod manager;


use std::io;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::collections::VecDeque;

use rotor::Notifier;

use carbon;


lazy_static! {
    pub static ref MANAGER: Manager = Manager::start();
}

pub struct Manager {
    thread: JoinHandle<Result<(), io::Error>>,
    notifier: Notifier,
    queue: Arc<Mutex<VecDeque<Seed>>>,
    pub carbon: carbon::Holder,
}

pub struct Context;


pub struct Main {
    queue: Arc<Mutex<VecDeque<Seed>>>,
}

pub enum Fsm {
    Main(Main),
    Carbon(carbon::Fsm<Context>),
}
pub enum Seed {
    Carbon(carbon::Seed),
}
