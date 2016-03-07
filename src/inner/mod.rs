mod start;
mod main;
mod manager;


use std::io;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::sync::atomic::AtomicUsize;
use std::collections::{VecDeque, HashMap};

use rotor::{Notifier, Machine};
use rotor::mio::tcp::TcpStream;

use carbon;
use http;


pub type SockId = usize; // because there is no atomic u64

lazy_static! {
    pub static ref MANAGER: Manager = Manager::start();
}

pub struct Manager {
    thread: JoinHandle<Result<(), io::Error>>,
    notifier: Notifier,
    queue: Arc<Mutex<VecDeque<Command>>>,
    sender: Arc<Mutex<Sender<Vec<u8>>>>,
    sockets: Arc<Mutex<HashMap<SockId, Socket>>>,
    id_gen: AtomicUsize,
    pub input: Arc<Mutex<Receiver<Vec<u8>>>>,
}

pub struct Context;


pub struct Main {
    queue: Arc<Mutex<VecDeque<Command>>>,
}

pub enum Fsm {
    Main(Main),
    Carbon(carbon::Fsm<Context, TcpStream>),
    Http(http::Fsm),
}
pub enum Command {
    NewCarbon(carbon::Seed),
    NewHttp(http::Seed),
    AcceptHttp(<http::Fsm as Machine>::Seed),
}

pub enum Socket {
    Carbon(carbon::Sink),
    HttpServer,
    HttpClient,
}
