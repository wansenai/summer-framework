use tonic::{transport::Server, Request, Response, Status};

use raft_rpc::appendentries_server::{AppendEntries, AppendEntriesServer};
use raft_rpc::{AppendEntriesRequest, AppendEntriesReply};

pub mod raft_rpc {
    tonic::include_proto!("grpc");
}