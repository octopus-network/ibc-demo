# IBC Demo

This project is to demonstrate the use of [pallet-ibc](https://github.com/cdot-network/substrate-ibc).

The runtime in this project is mainly composed of two special pallets.

One is pallet-ibc and the other is a template pallet that is defined in the [`pallets/template`](./pallets/template/src/lib.rs) directory.

pallet-ibc provides IBC support and the template pallet contains the demo logic of the chain.

And here is a [glue package](https://github.com/cdot-network/ibc-demo/tree/master/calls) that allows an RPC client based on substrate-subxt to interact with the demo chain through RPC.

The repository also includes implementation of [relayer process](https://github.com/cdot-network/ibc-demo/tree/master/relayer)(defined in [ICS 018](https://github.com/cosmos/ics/tree/master/spec/ics-018-relayer-algorithms)) and a [cli tool](https://github.com/cdot-network/ibc-demo/tree/master/cli) to make the cross-chain work.

## Local Development

Follow these steps to prepare a local Substrate development environment :hammer_and_wrench:

### Simple Setup

Install all the required dependencies with a single command (be patient, this can take up to 30
minutes).

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

### Manual Setup

Find manual setup instructions at the
[Substrate Developer Hub](https://substrate.dev/docs/en/knowledgebase/getting-started/#manual-installation).

### Build

Once the development environment is set up, build the node template. This command will build the
[Wasm](https://substrate.dev/docs/en/knowledgebase/advanced/executor#wasm-execution) and
[native](https://substrate.dev/docs/en/knowledgebase/advanced/executor#native-execution) code:

```bash
git clone https://github.com/cdot-network/ibc-demo.git
cd ibc-demo
git submodule update --init
cargo build --release
```

## Run

Start demo chains and send packet via IBC protocol:

```bash
./target/release/node-template --base-path /tmp/chain-appia --dev
./target/release/node-template --base-path /tmp/chain-flaminia --dev --port 20333 --ws-port 8844
./target/release/cli appia create-client flaminia
./target/release/cli flaminia create-client appia
./target/release/cli appia bind-port bank
./target/release/cli flaminia bind-port bank
./target/release/cli appia release-port bank // don't
export RUST_LOG=relayer=info
./target/release/relayer -c relayer/config.toml
./target/release/cli appia conn-open-init 53a954d6a7b1c595e025226e5f2a1782fdea30cd8b0d207ed4cdb040af3bfa10 779ca65108d1d515c3e4bc2e9f6d2f90e27b33b147864d1cd422d9f92ce08e03
./target/release/cli appia chan-open-init d93fc49e1b2087234a1e2fc204b500da5d16874e631e761bdab932b37907bd11 bank bank
./target/release/cli appia send-packet 1 1000 bank 00e2e14470ed9a017f586dfe6b76bb0871a8c91c3151778de110db3dfcc286ac bank a1611bcd0ba368e921b1bd3eb4aa66534429b14837725e8cef28182c25db601e 01020304
```
