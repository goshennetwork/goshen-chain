#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use riscv_evm;

pub const FIELD_ELEMENTS_PER_BLOB: usize = 4096;
pub const BLOB_COMMITMENT_VERSION_KZG: u8 = 0x01;
pub const BYTES_PER_BLOB: usize = 31 * FIELD_ELEMENTS_PER_BLOB;

pub struct BlobFetcher;
impl BlobFetcher {
    ///get blob from specific version
    fn get_element(version: [u8; 32], index: usize) -> [u8; 32] {
        riscv_evm::runtime::blob_at(version, index)
    }

    //version_hashes and blob num should have been already checked before.blob num is checked by l1 contract that always >0
    pub fn decode_version_hashes(blob_num: usize, version_hashes: &[u8]) -> Option<Vec<u8>> {
        let mut ret = Vec::new();
        let mut i = 0usize;
        for chunk in version_hashes.chunks_exact(32) {
            if i >= blob_num {
                break;
            }
            for j in 0..FIELD_ELEMENTS_PER_BLOB {
                //should never panic with unwrap method
                let element = BlobFetcher::get_element(chunk.try_into().unwrap(), j);
                //last byte should be zero, check it.
                if element[31] != 0 {
                    return None;
                }
                ret.extend_from_slice(&element[..31]);
            }
            //record blob num
            i += 1;
        }
        //now slice data
        let mut data = ret.split_off(4);
        //the length of data is big endian code
        let data_len = u32::from_be_bytes(ret.try_into().unwrap()) as usize;
        if data_len > data.len() {
            //wrong data len
            return None;
        }
        data.truncate(data_len);
        Some(data)
    }
}
