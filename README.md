# Rinux

## OS written in rust

### Pre-built binnaries:

LTS in [`/release`](/release/)

### Build yourself:

**First Run:**

```shell
cargo install bootimage
```

**Every time:**

```shell
cargo build --release
```

your finall binary should be in [`/target/x86_64-rinux/release/`](./target/x86_64-rinux/release/)

### Limitations:

no `std`

no terminal (yet, maybe)
