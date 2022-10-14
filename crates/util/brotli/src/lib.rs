#![no_std]
extern crate alloc;

mod heap;

use crate::heap::BrotliAlloc;
use alloc::vec;
use alloc::vec::Vec;
use brotli_decompressor::{BrotliDecompressStream, BrotliResult, BrotliState};

#[derive(Debug, PartialEq, Eq)]
pub enum DecompressError {
    InputInvalid,
    InputTooShort,
    OutputTooLarge,
}

pub fn decompress(input: &[u8], output_limit: usize) -> Result<Vec<u8>, DecompressError> {
    let wanted_in_len: usize = input.len();
    let mut available_out: usize = output_limit;
    let mut available_in: usize = input.len();
    let mut input_offset: usize = 0;
    let mut output_offset: usize = 0;
    let mut written: usize = 0;
    let mut output = vec![0; output_limit];
    let mut brotli_state =
        BrotliState::new(BrotliAlloc::new(), BrotliAlloc::new(), BrotliAlloc::new());
    let result = BrotliDecompressStream(
        &mut available_in,
        &mut input_offset,
        input,
        &mut available_out,
        &mut output_offset,
        &mut output,
        &mut written,
        &mut brotli_state,
    );
    match result {
        BrotliResult::ResultSuccess => {
            output.truncate(written);
        }
        BrotliResult::NeedsMoreInput | BrotliResult::ResultFailure => {
            return Err(DecompressError::InputInvalid);
        }
        BrotliResult::NeedsMoreOutput => {
            return Err(DecompressError::OutputTooLarge);
        }
    }
    if input_offset != wanted_in_len {
        // ensure all input has been consumed
        return Err(DecompressError::InputInvalid);
    }

    Ok(output)
}

#[test]
fn test_10x10y() {
    let input: [u8; 12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];
    let output = decompress(&input[..], 20).unwrap();
    let mut i: usize = 0;
    while i < 10 {
        assert_eq!(output[i], 'X' as u8);
        assert_eq!(output[i + 10], 'Y' as u8);
        i += 1;
    }
}

#[test]
fn test_limit_output() {
    let input: [u8; 12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];
    let output = decompress(&input[..], 19);
    assert_eq!(output.unwrap_err(), DecompressError::OutputTooLarge);
}
