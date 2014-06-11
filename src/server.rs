// Allow Ipv4Addr (only used in unit-tests)
#[allow(unused_imports)]
use std::io::net::ip::{SocketAddr, IpAddr, Ipv4Addr};
use http::server::{Config, Server, Request, ResponseWriter};

/// Your server will run under an instance of Gate
///
/// That instance will need access to the listen function,
/// so be sure to grab the trait: use oauth::server::Listener
pub struct Gate {
  pub ip: IpAddr,
  pub port: u16,
  /// Define your server's (only) handler
  ///
  /// See rust-http for more verbose examples
  pub handler: fn(&Request, &mut ResponseWriter)
}

impl Clone for Gate {
  fn clone(&self) -> Gate { *self }
}

impl Server for Gate {
  fn get_config(&self) -> Config {
    Config{ bind_address: SocketAddr{ ip: self.ip, port: self.port } }
  }

  fn handle_request(&self, req: &Request, res: &mut ResponseWriter) {
    debug!("Who goes there? (handling a gated query)")
    // Authorize

    let handler = self.handler;
    handler(req, res);
  }
}

pub trait Listener: Server {
  /// Call gate.listen() to kick it off!
  fn listen(self) {
    debug!("Serving from the Gate! (oAuth is enabled)")
    self.serve_forever();
  }
}

impl Listener for Gate {}

#[test]
fn configures() {
  fn handler(_req: &Request, _res: &mut ResponseWriter) {}

  let server = Gate{
    ip: Ipv4Addr(127, 0, 0, 1),
    port: 8001,
    handler: handler
  };

  server.get_config();
}
