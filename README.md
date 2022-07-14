<div align=center>
<img src="./docs/logo-300.png" width=300" height="300" />
</div>
<div align=center>
<img src="https://img.shields.io/badge/rust-1.54.0--nightly-blue"/>
<img src="https://img.shields.io/badge/substrate-3.0.0-lightBlue"/>
</div>

[github](https://github.com/hamster-shared/hamster): https://github.com/hamster-shared/hamster

hamster is a blockchain-based blockchain infrastructure service. Any computing device can easily access the Hamster network.

# Project Guidelines

## 1. Basic Introduction

### 1.1 Project Introduction

> Hamster Nodes are the underlying chain nodes of the Hamster Network and provide services to the entire Hamster Network.Hamster node is a custom node built on Substrate framework . It provides basic functions such as provider registration, calculating market, and executing orders.
>

### 1.2 Contributing Guide

Hi! Thank you for choosing Hamster.

Hamster is a blockchain that providers infrastructure service.

We are excited that you are interested in contributing to Hamster. Before submitting your contribution though, please make sure to take a moment and read through the following guidelines.

#### 1.2.1 Issue Guidelines

- Issues are exclusively for bug reports, feature requests and design-related topics. Other questions may be closed directly.

- Before submitting an issue, please check if similar problems have already been issued.

#### 1.2.2 Pull Request Guidelines

- Fork this repository to your own account. Do not create branches here.

- Commit info should be formatted as `[File Name]: Info about commit.` (e.g. `README.md: Fix xxx bug`)

- If your PR fixes a bug, please provide a description about the related bug.

- Merging a PR takes two maintainers: one approves the changes after reviewing, and then the other reviews and merges.

### 1.3 Version list

- main: 1.0.0 code, for prod
- develop: 2.0.0 dev code, for test

## 2. Getting started

### 2.1 Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### 2.2 Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### 2.3 Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### 2.4 Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-template -h
```

## 3. Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-template -lruntime=debug --dev
```

### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your local node template.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to
[our Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).

## 4. CodeStructure
```
├── docs                                docs
├── node                                substrate node module package
│   └── src                       substrate nodesource package
├── pallets                             substrate pallets package
│   ├── burn                      erc20 exchange with eth package 
│   │   └── src             erc20 exchange with eth implementation source code
│   ├── gateway                   hamster gateway package 
│   │   └── src             hamster gateway registry impl
│   ├── provider                  computing provides contract packages
│   │   └── src             Computing provides contract implementation source code
│   ├── resource-order            resource order contract package
│   │   └── src             resource order contract implementation template
│   └── template                  substrate pallet template
│       └── src                   substrate pallet template hello-world case  
├── primitives                          public object package
│   └── src                       public object source package
├── runtime                             substrate runtime package
│   └── src                       substrate runtime implementation package
└── scripts                             substrate run tool script directory       
```

## 5. Features

- provider: Provide functions such as registering resources, modifying the unit price of resources, adding rental hours and deleting resources
- resource-order: Provide functions for purchasing resources, executing orders, heartbeat reporting, pledge amounts, retrieving rewards, cancelling orders, renewing orders, etc.

## 6. Knowledge base

### 6.1 Team blog

> https://github.com/hamster-shared

## 7. Contributors

Thank you for considering your contribution to hamster!

<a href="https://github.com/hamster-shared/hamster/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=hamster-shared/hamster" />
</a>

## 8. Commercial considerations

If you use this project for commercial purposes, please comply with the Apache2.0 agreement and retain the author's technical support statement.