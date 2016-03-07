use std::net::SocketAddr;

use rotor::{Scope, Response, Void};
use rotor::mio::tcp::{TcpListener, TcpStream};
use rotor_http::ServerFsm;

use inner::Context;

pub mod ffi;
mod fsm;

pub type Seed = SocketAddr;
pub type Fsm = ServerFsm<fsm::BufferedHandler, TcpListener>;

pub fn create(addr: SocketAddr, scope: &mut Scope<Context>)
    -> Response<Fsm, Void>
{
    let sock = TcpListener::bind(&addr).expect("stator http bind");
    Fsm::new(sock, scope)
}
