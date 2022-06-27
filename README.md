# RustyOS

## OS written in rust

### Pre-built binnaries:

LTS in [`/release`](/release/)

### Build yourself:

**! IMPORTANT ! you need to have [bootimage](https://github.com/rust-osdev/bootimage) installed**

```shell
cargo build

cargo bootimage
```

your finall binary should be in [`/target/x86_64-rustyos/debug/`](./target/x86_64-rustyos/debug/)
