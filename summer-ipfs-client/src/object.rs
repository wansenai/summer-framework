use crate::IpfsApi;

use reqwest;
use serde_json;
use failure::Error;

#[derive(Deserialize, Debug, PartialEq, Hash)]
#[serde(rename_all="PascalCase")]
pub struct ObjectStats {
    hash: String,
    num_links: u64,
    block_size: u64,
    links_size: u64,
    data_size: u64,
    cumulative_size: u64
}

impl ObjectStats {
    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    pub fn num_links(&self) -> u64 {
        self.num_links
    }

    pub fn block_size(&self) -> u64 {
        self.block_size
    }

    pub fn links_size(&self) -> u64 {
        self.links_size
    }

    pub fn cumulative_size(&self) -> u64 {
        self.cumulative_size
    }
}

impl IpfsApi {
    pub fn object_stats(&self, hash: &str) -> Result<ObjectStats, Error> {
        let url = format!("http://{}:{}/api/v0/object/stat?arg={}", self.server, self.port, hash);
        let resp = reqwest::get(&url)?;
        Ok(serde_json::from_reader(resp)?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_object_stats() {
        let api = IpfsApi::new("127.0.0.1", 5001);
        let stats = api.object_stats("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u").unwrap();
        
        assert_eq!(stats.hash(), "QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u".to_string());
        assert_eq!(stats.cumulative_size(), 20);
    }
}