
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

use rocksdb::DB;
use rocksdb::rocksdb_options::UnsafeSnap;

pub struct Snapshot {
    db: Arc<DB>,
    snap: UnsafeSnap,

}

impl Snapshot{
    pub fn new(db: Arc<DB>) -> Snapshot {
        unsafe{
            Snapshot{
                snap: db.unsafe_snap(),
                db,
            }
        }
    }
}

impl Debug for Snapshot {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Jung Raft Store Snapshot")
    }
}

#[derive(Debug, Clone)]
pub struct SyncSnapshot(Arc<Snapshot>);

impl SyncSnapshot{
    pub fn new(db: Arc<DB>) -> SyncSnapshot { SyncSnapshot(Arc::new(Snapshot::new(db)))}

    pub fn clone(&self) -> SyncSnapshot {SyncSnapshot(Arc::clone(&self.0))}
}

#[cfg(test)]
mod tests {

}
