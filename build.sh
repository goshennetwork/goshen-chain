cd bin/riscv
cargo build --release --no-default-features -Z build-std=core,alloc --target ../../riscv32ima-unknown-none-elf.json --bin riscv-l2chain
cd ../../
cp target/riscv32ima-unknown-none-elf/release/riscv-l2chain .