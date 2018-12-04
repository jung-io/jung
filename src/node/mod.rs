use crossbeam::{unbounded, Receiver, Sender};

pub mod transport;
pub mod sender;
pub mod receiver;
pub mod raft_client;

pub enum StoreMessage {
    Propose {
        request: Request,
        cb: RequestCallback,
    },
    Raft(RaftMessage),
    ReportUnreachable(u64),
    ReportSnapshot {
        id: u64,
        status: SnapshotStatus,
    },
}