use crate::rpc::client::raft_rpc::append_entries_client::{AppendEntriesClient};
use crate::rpc::client::raft_rpc::{AppendEntriesRequest};
use crate::state::node::Node;

pub mod raft_rpc {

    tonic::include_proto!("rpcservice");

    async fn append_entries_request(node:Node,pending:Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = AppendEntriesClient::connect("http://127.0.0.1:50051").await?;
    
        let request = tonic::Request::new(AppendEntriesRequest {
            term : node.term().into(),
            leader_id : node.id().into(),
            prev_log_index : node.prev_log_index().into(),
            prev_log_term : node.prev_log_term().into(),
            entries : pending.into(),
            leader_commit : node.commit_index().into()
        });
    
        let response = client.request(request).await?;
    
        println!("RESPONSE={:?}", response);
    
        Ok(())
    }
}