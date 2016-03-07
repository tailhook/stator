use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::sync::atomic::AtomicIsize;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use rotor::{Scope, Response, Void};
use rotor::mio::tcp::TcpStream;
use rotor_carbon::{Sink as Carbon, connect_ip};

use inner::{Context};

pub mod ffi;

pub use rotor_carbon::Fsm;
pub type Seed = (SocketAddr, Sender<Sink>);
pub type Sink = Carbon<Context, TcpStream>;

pub fn create((addr, sender): Seed, scope: &mut Scope<Context>)
    -> Response<Fsm<Context, TcpStream>, Void>
{
    connect_ip(addr, scope).wrap(|(fsm, sink)| {
        sender.send(sink).expect("send sink to peer");
        fsm
    })
}
