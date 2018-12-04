use crossbeam::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use kvproto::raft_serverpb::RaftMessage;
use node::StoreMessage;
use std::sync::Arc;
use std::sync::RwLock;
use node::raft_client::RaftClient;

pub struct NodeTransport<T>
    where
        T: RaftRouter
{
    raftClient: Arc<RwLock<RaftClient>>
}

pub trait RaftRouter: Clone {
    fn send(&self, message: StoreMessage);
}
