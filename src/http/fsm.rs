use rotor::{Scope, Time};
use rotor_http;
use rotor_http::server::{Server, Response, Head, RecvMode};

use inner::Context;

impl rotor_http::server::Context for Context { }

pub struct BufferedHandler {
    a: u32
}

impl Server for BufferedHandler {
    type Context = Context;
    fn headers_received(head: Head, response: &mut Response,
        scope: &mut Scope<Self::Context>)
        -> Option<(Self, RecvMode, Time)>
    {
        unimplemented!();
    }
    fn request_received(self, data: &[u8], response: &mut Response,
        scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        unimplemented!();
    }
    fn request_chunk(self, chunk: &[u8], response: &mut Response,
        scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        unimplemented!();
    }
    fn request_end(self, response: &mut Response,
        scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        unimplemented!();
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
