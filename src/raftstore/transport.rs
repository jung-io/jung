use kvproto::raft_serverpb::RaftMessage;

pub trait Transport: Clone {
    fn send(&self, message: RaftMessage)-> () ;
}