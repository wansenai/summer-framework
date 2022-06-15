use std::io::{BufReader, BufRead};

use crate::IpfsApi;

use reqwest;
use serde_json;
use base64;
use failure::Error;

#[derive(Deserialize)]
struct JsonPubSubMessage {
    data: String,
    from: String,
    seqno: String
}

#[derive(Debug)]
pub struct PubSubMessage {
    data: Option<Vec<u8>>,
    from: Option<Vec<u8>>,
    seqno: Option<Vec<u8>>
}

impl PubSubMessage {
    pub fn data(&self) -> Option<Vec<u8>> {
        self.data.clone()
    }

    pub fn from(&self) -> Option<Vec<u8>> {
        self.from.clone()
    }

    pub fn seqno(&self) -> Option<Vec<u8>> {
        self.seqno.clone()
    }
}

impl IpfsApi {
    pub fn pubsub_subscribe(&self, channel: &str) -> Result<impl Iterator<Item=PubSubMessage>, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pubsub/sub");
        url.query_pairs_mut()
            .append_pair("arg", channel)
            .append_pair("discover", "true");
        let resp = reqwest::get(url)?;

        let messages = BufReader::new(resp).lines()
            .filter(|x|x.is_ok())
            .map(|x|x.unwrap())
            .map(|x|serde_json::from_str::<JsonPubSubMessage>(&x))
            .filter(|x|x.is_ok())
            .map(|x|x.unwrap())
            .map(|x| {
                PubSubMessage {
                    from: base64::decode(&x.from).ok(),
                    seqno: base64::decode(&x.seqno).ok(),
                    data: base64::decode(&x.data).ok()
                }
            });

        Ok(messages)
    }

    pub fn pubsub_publish(&self, channel: &str, data: &str) -> Result<(), Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pubsub/pub");
        url.query_pairs_mut()
            .append_pair("arg", channel)
            .append_pair("arg", data);
        let _resp = reqwest::get(url)?;
        Ok(())
    }
}