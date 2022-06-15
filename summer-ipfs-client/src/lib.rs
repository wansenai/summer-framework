extern crate reqwest;
extern crate serde_json;
extern crate serde;
extern crate failure;
extern crate base64;

#[macro_use] 
extern crate serde_derive;

mod cat;
mod ipns;
mod object;
pub mod pin;
pub mod pubsub;
mod version;
mod shutdown;
mod log;
mod block;

pub struct IpfsApi {
    server: String,
    port: u16
}


impl IpfsApi {
    pub fn new(server: &str, port: u16) -> IpfsApi {
        IpfsApi {
            server: server.into(),
            port: port
        }
    }

    fn get_url(&self) -> Result<reqwest::Url, reqwest::UrlError> {
        let url_string = format!("http://{}:{}/", self.server, self.port);
        reqwest::Url::parse(&url_string)
    }
}