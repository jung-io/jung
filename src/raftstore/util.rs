use kvproto::metapb;
use raftstore::replica::Replica;

pub fn find_peer(shard: &metapb::Region, store_id: u64) -> Option<&metapb::Peer>{
    shard
        .get_peers()
        .iter()
        .find(|&p| p.get_store_id() == store_id)
}