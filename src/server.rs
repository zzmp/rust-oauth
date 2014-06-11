use std::io::net::ip::{SocketAddr, IpAddr};
use http::server::{Config, Server, Request, ResponseWriter};

pub struct Gate {
  pub ip: IpAddr,
  pub port: u16,
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
  fn listen(self) {
    debug!("Serving from the Gate! (oAuth is enabled)")
    self.serve_forever();
  }
}

impl Listener for Gate {}
