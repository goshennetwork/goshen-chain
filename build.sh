cargo build --release -Z build-std=core,alloc --target ./riscv32ima-unknown-none-elf.json --no-default-features --bin riscv-l2chain
cp target/riscv32ima-unknown-none-elf/release/riscv-l2chain .