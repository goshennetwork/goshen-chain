use std::fs;
use std::str::FromStr;

use ethereum_types::H256;
use rustc_hex::FromHex;

use clap::Parser;
use common_types::bytes::ToPretty;
use ethcore::state::backend::ProofCheck;
use goshen_chain::state_transition;
use hash_db::HashDB;

#[derive(Parser)]
#[command(name = "Goshen Chain Sequencer Checker")]
#[command(author = "Goshen Team")]
#[command(version = "1.0")]
struct Cli {
    #[arg(long, short)]
    input: String,
    #[arg(long, short)]
    output: String,
}

fn main() {
    let contents = fs::read_to_string("./batch.data").unwrap();
    let mut db = ProofCheck::new(&[]);
    for line in contents.lines() {
        let data: Vec<u8> = line.from_hex().unwrap();
        db.insert(&data);
    }
    let cli: Cli = Cli::parse();
    let input = if cli.input.starts_with("0x") { &cli.input[2..] } else { &cli.input };
    let output = if cli.output.starts_with("0x") { &cli.output[2..] } else { &cli.output };

    let hash = H256::from_str(input).unwrap();
    let header = state_transition(db, hash);
    assert_eq!(output, header.to_hex());
}
