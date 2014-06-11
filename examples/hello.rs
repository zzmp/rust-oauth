extern crate time;
extern crate http;
extern crate oauth;

use oauth::server::{Gate, Listener};
use std::io::net::ip::Ipv4Addr;
use http::server::{Request, ResponseWriter};
use http::headers::content_type::MediaType;



fn main() {
  fn handler(_req: &Request, res: &mut ResponseWriter) {
      res.headers.date = Some(time::now_utc());
      res.headers.content_length = Some(14);
      res.headers.content_type = Some(MediaType {
        type_: String::from_str("text"),
        subtype: String::from_str("plain"),
        parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
      });
      res.headers.server = Some(String::from_str("Example"));

      res.write(bytes!("Hello, World!\n")).unwrap();
    }

  let server = Gate{
    ip: Ipv4Addr(127, 0, 0, 1),
    port: 8001,
    handler: handler
  };

  server.listen();
}

