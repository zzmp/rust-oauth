// Allow Ipv4Addr (only used in unit-tests)
#[allow(unused_imports)]
use std::io::net::ip::{SocketAddr, IpAddr, Ipv4Addr};
use url::Url;
use http::server::{Config, Server, Request, ResponseWriter};
use http::status::TemporaryRedirect;

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

  #[allow(unreachable_code)]
  fn handle_request(&self, _req: &Request, res: &mut ResponseWriter) {
    debug!("Who goes there? (handling a gated query)")
    // Authorize
    {
      res.status = TemporaryRedirect;
      res.headers.location = Some(
        Url {
          scheme:   "https".to_str(),
          user:     None,
          host:     "google.com".to_str(),
          port:     None,
          path:     "/is/cool".to_str(),
          query:    vec!(),
          fragment: None
        }
      );
      return;
    }

    let handler = self.handler;
    handler(_req, res);
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
