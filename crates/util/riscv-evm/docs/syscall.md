 # System Call
 
## Input/Output

```rust
pub fn input(hash: *mut u8); 
pub fn ret(hash: *const u8) -> !;
```

## HashDB
```rust
pub fn preimage_at(hash: *const u8, offset: usize) -> u32;
pub fn preimage_len(hash: *const u8) -> usize;
```

##  Debug/Panic
```rust
pub fn debug(msg: *const u8, len: usize);
pub fn panic(msg: *const u8, len: usize) -> !;
```

## EVM BuiltIn
```rust
/// hash, r, s: [u8;32], v: 0 or 1
/// result: [u8;20]
pub fn ecrecover(result: *mut u8, hash: *const u8, r: *const u8, s: *const u8, v: u32);
```