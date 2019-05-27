# rust-ckb-contract
This is a demo, write [ckb](https://github.com/nervosnetwork/ckb) smart contract with Rust.

**This project is just a POC.**

### tool chain

Please install nightly rust, because we need inline asm.

Add RISC-V target

```shell
rustup target add riscv64gc-unknown-none-elf
```

### compile

```shell
cargo build --target riscv64gc-unknown-none-elf
```

### run

```shell
# spike ./pk target/riscv64gc-unknown-none-elf/debug/demo
bbl loader
debug: Hello World!
```

The pk is modified version, see [here](https://github.com/rink1969/riscv-pk/tree/ckb-pk).

### note

Rust has no std for RISC-V now.

