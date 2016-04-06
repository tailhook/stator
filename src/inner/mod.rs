mod start;
mod main;
mod manager;
mod eventfd;
pub mod ffi;


use std::io;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::sync::atomic::AtomicUsize;
use std::collections::{VecDeque, HashMap};

use rotor::{Notifier, Machine};
use rotor::mio::tcp::TcpStream;

use carbon;
use http;
use redis;


pub type SockId = usize; // because there is no atomic u64

lazy_static! {
    pub static ref MANAGER: Manager = Manager::start();
}

pub struct Manager {
    thread: JoinHandle<Result<(), io::Error>>,
    notifier: Notifier,
    queue: Arc<Mutex<VecDeque<Command>>>,
    sockets: Arc<Mutex<HashMap<SockId, Socket>>>,
    id_gen: AtomicUsize,
    input: Arc<Mutex<VecDeque<(SockId, Box<[u8]>)>>>,
    input_notifier: eventfd::Async,
}

pub struct Context;


pub struct Main {
    queue: Arc<Mutex<VecDeque<Command>>>,
}

pub enum Fsm {
    Main(Main),
    Carbon(carbon::Fsm<Context, TcpStream>),
    Http(http::Fsm),
    Redis(redis::Fsm<TcpStream>),
}
pub enum Command {
    NewCarbon(carbon::Seed),
    NewHttp(http::Seed),
    NewRedis(redis::Seed),
    AcceptHttp(<http::Fsm as Machine>::Seed),
}

pub enum Socket {
    Carbon(carbon::Sink),
    HttpServer,
    HttpRequest(Arc<Mutex<Option<Box<[u8]>>>>, Notifier),
    Redis(redis::Redis<TcpStream>)
}
