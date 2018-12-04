use raftstore::config::Config;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use kvproto::metapb;
use raftstore::store::Store;
use raftstore::util;
use raft::RawNode;

pub struct Replica{
    cfg: Rc<Config>,
    replica_cache: RefCell<HashMap<u64, metapb::Peer>>,
    pub replica: metapb::Peer,
    shard_id: u64,
}

impl Replica{
    pub fn create<T>(store: &mut Store<T>, shard: &metapb::Region) -> Replica {
        let store_id = store.store_id();
        let meta_peer = match util::find_peer(shard, store_id) {
            None => {
                return Err(format!(
                    "find no peer for store {} in region {:?}",
                    store_id,
                    shard
                ))
            }
            Some(peer) => peer.clone(),
        };

        info!(
            "[region {}] create peer with id {}",
            region.get_id(),
            meta_peer.get_id(),
        );
        Replica::new(store, shard, meta_peer)
    }

    // The peer can be created from another node with raft membership changes, and we only
    // know the region_id and peer_id when creating this replicated peer, the region info
    // will be retrieved later after applying snapshot.
    pub fn replicate<T>(
        store: &mut Store<T>,
        region_id: u64,
        peer: metapb::Peer,
    ) -> Replica {

        let mut shard = metapb::Region::new();
        shard.set_id(region_id);
        Replica::new(store, &shard, peer)
    }

    fn new<T>(
        store: &mut Store<T>,
        shard: &metapb::Region,
        peer: metapb::Peer,
    ) -> Replica {
        if peer.get_id() == raft::INVALID_ID {
            return Err("Invalid Replica ID");
        }

        let cfg = store.config();

        let store_id = store.store_id();
        let sched = store.snap_scheduler();
        let tag = format!("[region {}] {}", shard.get_id(), peer.get_id());

        // TODO
        let ps = PeerStorage::new(
            store.engines(),
            shard,
            sched,
            tag.clone(),
            Rc::clone(&store.entry_cache_metries),
        )?;

        let applied_index = ps.applied_index();

        let raft_cfg = raft::Config {
            id: peer.get_id(),
            peers: vec![],
            election_tick: cfg.raft_election_timeout_ticks,
            heartbeat_tick: cfg.raft_heartbeat_ticks,
            min_election_tick: cfg.raft_min_election_timeout_ticks,
            max_election_tick: cfg.raft_max_election_timeout_ticks,
            max_size_per_msg: cfg.raft_max_size_per_msg.0,
            max_inflight_msgs: cfg.raft_max_inflight_msgs,
            applied: applied_index,
            check_quorum: true,
            tag: tag.clone(),
            skip_bcast_commit: true,
            pre_vote: cfg.prevote,
            ..Default::default()
        };

        let raft_group = RawNode::new(&raft_cfg, ps, vec![])?;
        let mut replica = Replica {
            engines: store.engines(),
            peer,
            region_id: region.get_id(),
            raft_group,
            proposals: Default::default(),
            apply_proposals: vec![],
            pending_reads: Default::default(),
            peer_cache: RefCell::new(HashMap::default()),
            peer_heartbeats: HashMap::default(),
            peers_start_pending_time: vec![],
            coprocessor_host: Arc::clone(&store.coprocessor_host),
            size_diff_hint: 0,
            delete_keys_hint: 0,
            approximate_size: None,
            approximate_keys: None,
            compaction_declined_bytes: 0,
            apply_scheduler: store.apply_scheduler(),
            read_scheduler: store.read_scheduler(),
            pending_remove: false,
            marked_to_be_checked: false,
            pending_merge_state: None,
            last_committed_prepare_merge_idx: 0,
            leader_missing_time: Some(Instant::now()),
            tag,
            last_applying_idx: applied_index,
            last_compacted_idx: 0,
            last_urgent_proposal_idx: u64::MAX,
            last_committed_split_idx: 0,
            consistency_state: ConsistencyState {
                last_check_time: Instant::now(),
                index: INVALID_INDEX,
                hash: vec![],
            },
            raft_log_size_hint: 0,
            raft_entry_max_size: cfg.raft_entry_max_size.0,
            leader_lease: Lease::new(cfg.raft_store_max_leader_lease()),
            cfg,
            pending_messages: vec![],
            peer_stat: PeerStat::default(),
        };


        replica
    }
}