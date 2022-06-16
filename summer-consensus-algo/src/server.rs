use tonic::{transport::Server, Request, Response, Status};

use raft_rpc::append_entries_server::{AppendEntries, AppendEntriesServer};
use raft_rpc::{AppendEntriesRequest, AppendEntriesReply};

pub mod raft_rpc {
    tonic::include_proto!("rpcservice");
}

#[derive(Debug, Default)]
pub struct Heartbeat {}

#[tonic::async_trait]
impl AppendEntries for Heartbeat {
    async fn request(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = raft_rpc::AppendEntriesReply {
            term: format!("Hello {}!", request.into_inner().term).into(),
            success: "true".into()
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let heartbeat = Heartbeat::default();

    Server::builder()
        .add_service(AppendEntriesServer::new(heartbeat))
        .serve(addr)
        .await?;

    Ok(())
}