use crate::HashDBOracle;
use alloc::vec::Vec;
use byteorder::{BigEndian, ByteOrder};
use common_types::header::Header;
use common_types::l2_cfg::{L1_CROSS_LAYER_WITNESS, L2_BLOCK_MAX_GAS_LIMIT, MAX_SENDER_NONCE};
use common_types::transaction::TypedTxId::Legacy;
use common_types::transaction::{TypedTransaction, UnverifiedTransaction};
use ethcore::client::LastHashes;
use ethereum_types::H256;
use rlp::Rlp;

// format: queueNum(uint64) + queueStart(uint64) + batchNum(uint64) + batch0Time(uint64) +
// batchLeftTimeDiff([]uint32) + batchesData
fn load_batches_from_hashdb(db: &HashDBOracle, batch_input_hash: H256) -> Vec<Batch> {
    let raw_input = db.get(&batch_input_hash).expect("input not found");
    let raw_input = raw_input.into_vec();
    let batch_hash = H256::from_slice(&raw_input[..32]);
    let queue_hash = H256::from_slice(&raw_input[32..64]);
    let raw_batch = db.get(&batch_hash).expect("input batch not found");
    let raw_batch = raw_batch.into_vec();
    let queue_num = BigEndian::read_u64(&raw_batch[..8]) as usize;
    let batch_num = BigEndian::read_u64(&raw_batch[16..24]) as usize;
    let mut batches = Vec::with_capacity(queue_num + batch_num);
    let mut timestamps = Vec::with_capacity(batch_num);
    let queue_txes = load_queue_txes(db, queue_hash);
    batches.extend(
        queue_txes
            .iter()
            .map(|item| Batch { transactions: item.txs.clone(), timestamp: item.timestamp }),
    );

    if batch_num > 0 {
        let timeend = 24 + batch_num * 4 + 4;
        let time_slice = &raw_batch[24..timeend];
        let batches_slice = &raw_batch[timeend..raw_batch.len()];
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
    if num_batches != timestamp.len() {
        return Vec::new();
    }
    let mut batches = Vec::with_capacity(num_batches);
    for (batch, time) in rlp.iter().zip(timestamp) {
        let mut batch = Batch {
            timestamp: time,
            transactions: TypedTransaction::decode_rlp_list(&batch).expect("decode batch err"),
        };
        // ensure there are not enqueued tx in batch
        batch.transactions.retain(|tx| {
            let sender = tx.recover_sender().unwrap_or(L1_CROSS_LAYER_WITNESS);
            if sender == L1_CROSS_LAYER_WITNESS {
                return false;
            }
            let nonce = tx.tx().nonce.as_u64();
            if nonce >= MAX_SENDER_NONCE {
                return false;
            }
            if tx.tx_type() != Legacy {
                return false;
            }
            return true;
        });
        batches.push(batch);
    }

    batches
}

pub struct Batch {
    pub timestamp: u64,
    pub transactions: Vec<UnverifiedTransaction>,
}

pub struct QueueTxInfo {
    txs: Vec<UnverifiedTransaction>,
    timestamp: u64,
}

// TODO: merge txs that included at same block to one QueueTxInfo
fn load_queue_txes(db: &HashDBOracle, hash: H256) -> Vec<QueueTxInfo> {
    let raw = db.get(&hash).expect("queue preimage not found");
    let raw = raw.into_vec();
    let mut result: Vec<QueueTxInfo> = Vec::new();
    raw.chunks_exact(40).for_each(|chunk| {
        let txhash = H256::from_slice(&chunk[..32]);
        let timestamp = BigEndian::read_u64(&chunk[32..]);
        let raw = db.get(&txhash).expect("queue tx not found");
        let rlp = Rlp::new(&raw);
        let tx = TypedTransaction::decode_rlp(&rlp).unwrap();
        let q_info = result.iter_mut().rfind(|info| info.timestamp == timestamp);
        match q_info {
            None => {
                let mut txs = Vec::new();
                txs.push(tx);
                result.push(QueueTxInfo { timestamp, txs })
            }
            Some(q) => {
                let total_gas = q.txs.iter().fold(0, |gas, tx| gas + tx.tx().gas.as_u32());
                if total_gas > L2_BLOCK_MAX_GAS_LIMIT {
                    let mut txs = Vec::new();
                    txs.push(tx);
                    result.push(QueueTxInfo { timestamp, txs });
                } else {
                    q.txs.push(tx);
                }
            }
        }
    });
    result.sort_by_key(|i| i.timestamp);
    return result;
}

pub struct RollupInput {
    pub prev_header: Header,
    pub batches: Vec<Batch>,
}

impl RollupInput {
    pub fn load_from_hashdb(db: &HashDBOracle, entry_hash: H256) -> RollupInput {
        let raw = db.get(&entry_hash).expect("input not found");
        let batch_input_hash = H256::from_slice(&raw[..32]);
        let batches = load_batches_from_hashdb(db, batch_input_hash);
        let prev_block_hash = H256::from_slice(&raw[32..64]);
        let header = load_header(db, prev_block_hash);
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
