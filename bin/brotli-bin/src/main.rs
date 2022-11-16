extern crate alloc;
use alloc::format;
use alloc::vec::Vec;
use byteorder::{BigEndian, ByteOrder};
use common_types::transaction::TypedTransaction;
use rlp::Rlp;
use rustc_hex::FromHex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_batch: Vec<u8> = args[1].from_hex().unwrap();
    let batch_num = BigEndian::read_u64(&raw_batch[24..32]) as usize;
    let mut timestamps = Vec::with_capacity(batch_num);
    println!("batch num: {}", batch_num);
    if batch_num > 0 {
        let timeend = 32 + batch_num * 4 + 4;
        let time_slice = &raw_batch[32..timeend];
        let batches_slice = &raw_batch[timeend..raw_batch.len()];
        let mut time = BigEndian::read_u64(&time_slice[..8]);
        timestamps.push(time);
        for i in 1..batch_num {
            time += BigEndian::read_u32(&time_slice[4 + i * 4..]) as u64;
            timestamps.push(time);
        }
        if decode_batches(batches_slice, timestamps).is_none() {
            panic!("decode failed")
        }
    }
}

// verison(byte) + data
// v0: 0 + rlplist(rlplist(tx))
pub fn decode_batches(data: &[u8], timestamp: Vec<u64>) -> Option<bool> {
    println!("decode_batches");
    let version = data[0];
    if version > 1 {
        // invalid version
        panic!("wrong version")
    }
    let mut rlp = Rlp::new(&data[1..]);
    let mut d: Vec<u8>;
    if version == 1 {
        println!("decompress");
        // brotli version
        let ret = brotli::decompress(&data[1..], 4 * 1024 * 1024); // Now limit is 4MB
        if ret.is_err() {
            panic!("brtoli decompress failed")
        }
        d = ret.unwrap();
        rlp = Rlp::new(d.as_slice());
    }
    if !rlp.is_list() {
        //wrong rlp code
        panic!("rlp decode failed")
    }
    let num_batches = rlp.item_count().expect("expect batch list");
    for (batch, time) in rlp.iter().zip(timestamp) {
        let txs = TypedTransaction::decode_rlp_list(&batch);
        if !txs.is_ok() {
            //decode failed
            panic!("rlp decode failed")
        }
    }

    Option::Some(true)
}
