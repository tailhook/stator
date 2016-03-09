use std::io::Cursor;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use rotor::{Scope, Time};
use rotor_http;
use rotor_http::server::{Server, Response, Head, RecvMode};
use cbor::Encoder;

use inner::{MANAGER, Context, SockId};
use inner::Socket::HttpRequest;

impl rotor_http::server::Context for Context { }

pub enum BufferedHandler {
    HeadersRead(Encoder<Cursor<Vec<u8>>>),
    WaitingResponse(SockId, Arc<Mutex<Option<Box<[u8]>>>>),
}

impl Server for BufferedHandler {
    type Context = Context;
    fn headers_received(head: Head, _response: &mut Response,
        scope: &mut Scope<Self::Context>)
        -> Option<(Self, RecvMode, Time)>
    {
        let mut enc = Encoder::new(Cursor::new(Vec::new()));
        enc.object(8).unwrap();
        enc.text("client").unwrap();
        match head.client {
            Some(ip) => enc.text(&format!("{}", ip)).unwrap(),
            None => enc.null().unwrap(),
        }
        enc.text("version").unwrap();
        enc.text(&format!("{}", head.version)).unwrap();
        enc.text("method").unwrap();
        enc.text(head.method).unwrap();
        enc.text("scheme").unwrap();
        enc.text(head.scheme).unwrap();
        enc.text("path").unwrap();
        enc.text(head.path).unwrap();
        enc.text("headers").unwrap();
        enc.object(head.headers.len()).unwrap();
        for header in head.headers {
            enc.text(header.name).unwrap();
            enc.bytes(header.value).unwrap();
        }
        // And body and socket_id will be appended
        // as 7th and 8th elements in the request_received method
        Some((BufferedHandler::HeadersRead(enc),
             RecvMode::Buffered(1_048_576),
             scope.now() + Duration::new(120, 0)))
    }
    fn request_received(self, data: &[u8], _response: &mut Response,
        scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        match self {
            BufferedHandler::HeadersRead(mut enc) => {
                let arc = Arc::new(Mutex::new(None));
                let sock_id = MANAGER.insert(
                    HttpRequest(arc.clone(), scope.notifier()));

                enc.text("body").unwrap();
                enc.bytes(data).unwrap();
                enc.text("response_socket").unwrap();
                enc.u64(sock_id as u64).unwrap();

                let vec = enc.into_writer().into_inner();
                // TODO(tailhook) propagate socket number
                MANAGER.send(1, vec.into_boxed_slice());
                Some(BufferedHandler::WaitingResponse(sock_id, arc))
            }
            _ => unreachable!(),
        }
    }
    fn request_chunk(self, _chunk: &[u8], _response: &mut Response,
        _scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        unreachable!();
    }
    fn request_end(self, _response: &mut Response,
        _scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        unreachable!();
    }
    fn timeout(self, response: &mut Response, scope: &mut Scope<Self::Context>)
        -> Option<(Self, Time)>
    {
        unimplemented!();
    }
    fn wakeup(self, response: &mut Response, scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        unimplemented!();
    }

}
