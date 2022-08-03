# Stateset Contracts

Core smart contracts for implementing purchase order financing and invoice factoring on the Stateset Blockchain.

## Prerequisites

Before starting, make sure you have [rustup](https://rustup.rs/) along with a
recent `rustc` and `cargo` version installed. Currently, we are testing on 1.58.1+.

And you need to have the `wasm32-unknown-unknown` target installed as well.

You can check that via:

```sh
rustc --version
cargo --version
rustup target list --installed
# if wasm32 is not listed above, run this
rustup target add wasm32-unknown-unknown
```

# Contracts

Here are a number of useful contracts that either implement or consume
the interfaces defined in `packages/cw*`.

## Creating a new contract

Use [`cosmwasm-template`](https://github.com/CosmWasm/cosmwasm-template) as a basis

```bash
cd contracts
cargo generate --git https://github.com/CosmWasm/cosmwasm-template.git --name PROJECT_NAME
cd PROJECT_NAME
rm -rf .git
rm .gitignore
rm .cargo-ok
git add .
```

## Compiling and running tests

Now that you created your custom contract, make sure you can compile and run it before
making any changes. Go into the repository and do:

```sh
# this will produce a wasm build in ./target/wasm32-unknown-unknown/release/YOUR_NAME_HERE.wasm
cargo wasm

# this runs unit tests with helpful backtraces
RUST_BACKTRACE=1 cargo unit-test

# auto-generate json schema
cargo schema
```


