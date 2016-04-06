use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use rotor::{Scope, Response, Void, Notifier};
use rotor::mio::tcp::TcpStream;
use rotor_redis;
use rotor_stream::sync::Mutexed;
use rotor_stream::{Persistent, ActiveStream};

use inner::{Context, SockId, MANAGER, Socket};
use self::receiver::Receiver;

mod receiver;
pub mod ffi;

use rotor_redis::RedisProto;
pub type Seed = (SocketAddr, u32, Sender<SockId>);
pub type Fsm<S> = Mutexed<Persistent<RedisProto<Context, S, Receiver>>>;

pub struct Redis<S>(Arc<Mutex<Persistent<RedisProto<Context, S, Receiver>>>>,
                    Notifier)
    where S: ActiveStream;

/// TODO(tailhook) I'm not sure this a good idea
/// but couldn't get it working otherwise (it's perfectly safe anyway)
unsafe impl<S: ActiveStream> Send for Redis<S> {}

impl rotor_redis::Context for Context { }

pub fn create((addr, db, sender): Seed, scope: &mut Scope<Context>)
    -> Response<Fsm<TcpStream>, Void>
{
    let mut resp = None;
    let sock_id = MANAGER.insert_with(|sock_id| {
        let mut cell = None;
        resp = Some(
            Persistent::connect(scope, addr, (db, Receiver::new(sock_id)))
            .wrap(|fsm| {
                let arc = Arc::new(Mutex::new(fsm));
                cell = Some(Redis(arc.clone(), scope.notifier()));
                Mutexed(arc)
            }));
        Socket::Redis(cell.unwrap())
    });
    sender.send(sock_id).unwrap();
    return resp.unwrap();
}
