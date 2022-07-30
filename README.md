# Rinux

## OS written in rust

### Pre-built binnaries:

LTS in [`/release`](/release/)

### Build yourself:

**First Run:**

```shell
cargo install bootimage
```

**! IMPORTANT ! you need to have [bootimage](https://github.com/rust-osdev/bootimage) installed**

```shell
cargo build --release
```

your finall binary should be in [`/target/x86_64-rustyos/debug/`](./target/x86_64-rustyos/debug/)

### Limitations:

no `std`

no terminal

