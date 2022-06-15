use crate::IpfsApi;

use reqwest;
use failure::Error;

impl IpfsApi {
    pub fn shutdown(&self) -> Result<(), Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/shutdown");
        let _resp = reqwest::get(url)?;
        Ok(())
    }
}