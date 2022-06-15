use crate::IpfsApi;

use serde_json::Value;

use reqwest;
use serde_json;
use failure::err_msg;
use failure::Error;

impl IpfsApi {
    pub fn name_resolve(&self, name: &str) -> Result<String, Error> {
        let url = format!("http://{}:{}/api/v0/name/resolve?arg={}", self.server, self.port, name);
        let resp = reqwest::get(&url)?;
        let resp: Value = serde_json::from_reader(resp)?;
        
        if resp["Path"].is_string() {
            Ok(resp["Path"].as_str().unwrap().into())
        } else {
            Err(err_msg("Key error"))
        }
    }

    pub fn name_publish(&self, hash: &str) -> Result<(), Error> {
        let url = format!("http://{}:{}/api/v0/name/publish?arg={}", self.server, self.port, hash);
        let _resp = reqwest::get(&url)?;
        Ok(())
    }
}