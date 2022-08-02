# Rinux

## OS written in rust

### Build yourself:

Tools needed to run: [`enderpearl`](https://github.com/AtomicGamer9523/Enderpearl), [`bootimage`](https://github.com/rust-osdev/bootimage), and [`qemu`](https://qemu.org)

1. Modify project's metadata in [.config.enderpearl](./.config.enderpearl)

2. Import your project via [.build.enderpearl](./.build.enderpearl)

3. Build & Run:

```shell
enderpearl run
```

or

```shell
cargo run
```

<br>

### Limitations:

no `std`

no terminal (yet, maybe)

<br>

### Build for release:

```shell
enderpearl release
```
