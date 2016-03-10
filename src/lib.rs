#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rotor;
extern crate rotor_tools;
extern crate rotor_http;
extern crate rotor_carbon;
extern crate cbor;
extern crate env_logger;
#[macro_use] extern crate log;

mod inner;
mod carbon;
mod http;

pub use carbon::ffi::stator_carbon_connect_ipv4;
pub use carbon::ffi::stator_carbon_add_i64;
pub use carbon::ffi::stator_carbon_add_f64;
pub use carbon::ffi::stator_carbon_add_i64_at;
pub use carbon::ffi::stator_carbon_add_f64_at;

pub use http::ffi::stator_http_bind_ipv4;
pub use http::ffi::stator_http_reply;

pub use inner::ffi::stator_wait_message;
