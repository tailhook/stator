use std::net::SocketAddr;

use rotor::{Scope, Response, Void};
use rotor::mio::tcp::{TcpListener};
use rotor_http::server;

use inner::{Context, SockId};

pub mod ffi;
mod fsm;

pub type Seed = (SocketAddr, SockId);
pub type Fsm = server::Fsm<fsm::BufferedHandler, TcpListener>;

pub fn create((addr, id): (SocketAddr, SockId), scope: &mut Scope<Context>)
    -> Response<Fsm, Void>
{
    let sock = TcpListener::bind(&addr).expect("stator http bind");
    Fsm::new(sock, id, scope)
}
