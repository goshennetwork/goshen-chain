use crate::HashDBOracle;
use alloc::sync::Arc;
use alloc::vec::Vec;
use byteorder::{BigEndian, ByteOrder};
use common_types::header::Header;
use common_types::transaction::{SignedTransaction, TypedTransaction, UnverifiedTransaction};
use core::hash::Hash;
use ethcore::client::LastHashes;
use ethereum_types::{Address, H256};
use hash_db::HashDB;
use keccak_hasher::KeccakHasher;
use rlp::Rlp;

// format: queueNum(uint64) + queueStart(uint64) + batchNum(uint64) + batch0Time(uint64) +
// batchLeftTimeDiff([]uint32) + batchesData + queueInfoHash
// TODO: load queue tx
fn load_batches_from_hashdb(db: &HashDBOracle, hash: H256) -> Vec<Batch> {
    let raw = db.get(&hash).expect("input not found");
    let raw = raw.into_vec();
    let queue_num = BigEndian::read_u64(&raw[..8]) as usize;
    //let _queue_start = BigEndian::read_u64(&raw[8..16]);
    let batch_num = BigEndian::read_u64(&raw[16..24]) as usize;
    let mut batches = Vec::with_capacity(queue_num + batch_num);
    let mut timestamps = Vec::with_capacity(batch_num);
    assert!(raw.len() > 24 + 32);
    let queue_hash = H256::from_slice(&raw[raw.len() - 32..]);
    let queue_txes = load_queue_txes(db, queue_hash);

    if batch_num > 0 {
        let timeend = 24 + batch_num * 4 + 4;
        let time_slice = &raw[24..timeend];
        let batches_slice = &raw[timeend..raw.len() - 32];
        let mut time = BigEndian::read_u64(&time_slice[..8]);
        timestamps.push(time);
        for i in 1..batch_num {
            time += BigEndian::read_u32(&time_slice[4 + i * 4..]) as u64;
            timestamps.push(time);
        }
        batches.extend(decode_batches(batches_slice, timestamps));
    }
    batches.sort_by_key(|v| v.timestamp);

    batches
}

// verison(byte) + data
// v0: 0 + rlplist(rlplist(tx))
fn decode_batches(data: &[u8], timestamp: Vec<u64>) -> Vec<Batch> {
    let version = data[0];
    assert_eq!(version, 0, "unknown version");
    let rlp = Rlp::new(&data[1..]);
    assert!(rlp.is_list());
    let num_batches = rlp.item_count().expect("expect batch list");
    let mut batches = Vec::with_capacity(num_batches);
    for (batch, time) in rlp.iter().zip(timestamp) {
        let batch = Batch {
            timestamp: time,
            transactions: TypedTransaction::decode_rlp_list(&batch).expect("decode batch err"),
        };
        batches.push(batch);
    }

    batches
}

pub struct Batch {
    pub timestamp: u64,
    pub transactions: Vec<UnverifiedTransaction>,
}

pub struct QueueTxInfo {
    timestamp: u64,
    sender: Address,
    target: Address,
    gas_limit: u64,
    data: Vec<u8>,
}

fn load_queue_txes(db: &HashDBOracle, hash: H256) -> Vec<QueueTxInfo> {
    let raw = db.get(&hash).expect("queue preimage not found");
    let raw = raw.into_vec();
    raw.chunks(40)
        .map(|chunk| {
            let txhash = H256::from_slice(&chunk[..32]);
            let timestamp = BigEndian::read_u64(&chunk[32..]);
            let raw = db.get(&txhash).expect("queue tx not found");
            let sender = Address::from_slice(&raw[..20]);
            let target = Address::from_slice(&raw[20..40]);
            let gas_limit = BigEndian::read_u64(&raw[40..48]);
            let data = raw[48..].to_vec();
            QueueTxInfo { timestamp, sender, target, gas_limit, data }
        })
        .collect()
}

pub struct RollupInput {
    pub prev_header: Header,
    pub batches: Vec<Batch>,
}

impl RollupInput {
    pub fn load_from_hashdb(db: &HashDBOracle, hash: H256) -> RollupInput {
        let raw = db.get(&hash).expect("input not found");
        let prev_block_hash = H256::from_slice(&raw[..32]);
        let header = load_header(db, prev_block_hash);
        let batches = load_batches_from_hashdb(db, H256::from_slice(&raw[32..64]));
        RollupInput { prev_header: header, batches }
    }
}

pub fn load_header(db: &HashDBOracle, hash: H256) -> Header {
    let raw = db.get(&hash).expect("input not found");
    // TODO: eip1559 base fee
    Header::decode_rlp(&Rlp::new(&raw), u64::MAX).expect("load header err")
}

pub fn load_last_hashes(db: &HashDBOracle, mut hash: H256, height: u64) -> LastHashes {
    let mut hashes = Vec::with_capacity(256);
    hashes.push(hash);

    let count = if height < 255 { height } else { 255 };
    for _ in 0..count {
        let header = load_header(db, hash);
        hash = *header.parent_hash();
        hashes.push(hash);
    }

    hashes
}
