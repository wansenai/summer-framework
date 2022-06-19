use raft_rpc::append_entries_client::{AppendEntriesClient};
use raft_rpc::{AppendEntriesRequest};

pub mod raft_rpc {
    tonic::include_proto!("rpcservice");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AppendEntriesClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(AppendEntriesRequest {
        term : 1.into(),
        leader_id : "1".into(),
        prev_log_index : 1.into(),
        prev_log_term : 1.into(),
        entries : vec!["1".to_string()].into(),
        leader_commit : 1.into()
    });

    let response = client.request(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}