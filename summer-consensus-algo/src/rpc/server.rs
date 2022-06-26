use tonic::{transport::Server, Request, Response, Status};

use raft_rpc::append_entries_server::{AppendEntries, AppendEntriesServer};
use raft_rpc::{AppendEntriesRequest, AppendEntriesReply};

//从.proto文件解析协议
pub mod raft_rpc {
    tonic::include_proto!("rpcservice");
}

//定义rpc方法使用的实例
#[derive(Debug, Default)]
pub struct Heartbeat {}

//定义AppendEntries服务中的rpc操作
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
    //从基本库中解析监听的IP地址
    let addr = "127.0.0.1:50051".parse()?;
    let heartbeat = Heartbeat::default();

    //启动服务
    Server::builder()
        .add_service(AppendEntriesServer::new(heartbeat))
        .serve(addr)
        .await?;

    Ok(())
}