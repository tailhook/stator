use std::io::Cursor;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use rotor::{Scope, Time};
use rotor_http::server::{Server, Response, Head, RecvMode};
use cbor::{Encoder, Decoder, Config};

use inner::{MANAGER, Context, SockId};
use inner::Socket::HttpRequest;

pub enum BufferedHandler {
    HeadersRead { parent: SockId, buffer: Encoder<Cursor<Vec<u8>>> },
    WaitingResponse { id: SockId, response: Arc<Mutex<Option<Box<[u8]>>>> },
}

fn decode_into_response(data: &[u8], res: &mut Response)
    -> Result<(), String>
{
    let mut dec = Decoder::new(Config::default(), Cursor::new(data));
    let num = try!(dec.array()
        .map_err(|e| format!("must be an array: {}", e)));
    if num != 3 {
        return Err(format!("array must contain 3 elements: \
            status, headers, body; contains {} instead", num));
    }
    let num2 = try!(dec.array()
        .map_err(|e| format!("status must be an array: {}", e)));
    if num2 != 2 {
        return Err(format!("status array must contain 2 elements: \
            status_code and status_text; contains {} instead", num));
    }
    let status = try!(dec.u16()
        .map_err(|e| format!("status code must be number 200..599: {}", e)));
    if status < 200 || status > 599 {
        return Err(format!("status code must be number 200..599"));
    }
    {
        let text = try!(dec.text_borrow()
            .map_err(|e| format!("status text must be string: {}", e)));
        res.status(status, text);
    }
    let num3 = try!(dec.object()
        .map_err(|e| format!("headers must be dict (object): {}", e)));
    for _ in 0..num3 {
        // unfortunately can't borrow both
        let name = try!(dec.text()
            .map_err(|e| format!(
                "header name must be string (unicode): {}", e)));
        let value = try!(dec.bytes_borrow()
            .map_err(|e| format!(
                "header value must be bytes (binary): {}", e)));
        try!(res.add_header(&name, value)
            .map_err(|e| format!("error adding header: {}", e)));
    }
    let body = try!(dec.bytes_borrow()
        .map_err(|e| format!("response body must be bytes (binary): {}", e)));
    try!(res.add_length(body.len() as u64)
        .map_err(|e| format!("error adding response length: {}", e)));
    try!(res.done_headers()
        .map_err(|e| format!("error finalizing headers: {}", e)));
    res.write_body(body);
    res.done();
    Ok(())
}

fn write_502(response: &mut Response) {
    if !response.is_started() {
        response.status(502, "Bad Gateway");
        let data = "<h1>502 Bad Gateway</h1>\n\
            <p><small>Served for you by stator(rotor-http)</small></p>\n";
        let bytes = data.as_bytes();
        response.add_length(bytes.len() as u64).unwrap();
        response.add_header("Content-Type", b"text/html").unwrap();
        response.done_headers().unwrap();
        response.write_body(bytes);
        response.done();
    }
}

impl Server for BufferedHandler {
    type Seed = SockId;
    type Context = Context;
    fn headers_received(parent_socket: SockId,
        head: Head, _response: &mut Response,
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
        Some((BufferedHandler::HeadersRead { parent: parent_socket,
                                             buffer: enc },
             RecvMode::Buffered(1_048_576),
             scope.now() + Duration::new(120, 0)))
    }
    fn request_received(self, data: &[u8], _response: &mut Response,
        scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        match self {
            BufferedHandler::HeadersRead { parent, buffer: mut enc } => {
                let arc = Arc::new(Mutex::new(None));
                let sock_id = MANAGER.insert(
                    HttpRequest(arc.clone(), scope.notifier()));

                enc.text("body").unwrap();
                enc.bytes(data).unwrap();
                enc.text("response_socket").unwrap();
                enc.u64(sock_id as u64).unwrap();

                let vec = enc.into_writer().into_inner();
                MANAGER.send(parent, vec.into_boxed_slice());
                Some(BufferedHandler::WaitingResponse { id: sock_id,
                                                        response: arc })
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
    fn wakeup(self, resp: &mut Response, _scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        match self {
            BufferedHandler::WaitingResponse { id, response } => {
                let data = response.lock().expect("error").take();
                if let Some(data) = data {
                    match decode_into_response(&data[..], resp) {
                        Ok(()) => {}
                        Err(e) => {
                            error!("error in response for http request: {}",
                                e);
                            write_502(resp);
                        }
                    }
                    MANAGER.remove(id);
                    None
                } else {
                    Some(BufferedHandler::WaitingResponse {
                        id: id, response: response })
                }
            }
            me @ _ => Some(me)
        }
    }

}
