# rust-ckb-contract

### tool chain

Use nightly rust, because we need use inline asm.

add RISC-V target

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

pk is modified, see [here](https://github.com/rink1969/riscv-pk/tree/ckb-pk).

### note

There are no std for RISC-V.