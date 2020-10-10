# IBC Demo

This project is to demonstrate the use of [pallet-ibc](https://github.com/cdot-network/substrate-ibc).

The runtime in this project is mainly composed of two special pallets.

One is pallet-ibc and the other is a template pallet that is defined in the [`pallets/template`](./pallets/template/src/lib.rs) directory.

pallet-ibc provides IBC support and the template pallet contains the demo logic of the chain.

And here is a [glue package](https://github.com/cdot-network/ibc-demo/tree/master/calls) that allows an RPC client based on substrate-subxt to interact with the demo chain through RPC.

The repository also includes implementation of [relayer process](https://github.com/cdot-network/ibc-demo/tree/master/relayer) defined in [ICS 018](https://github.com/cosmos/ics/tree/master/spec/ics-018-relayer-algorithms) and a [cli tool](https://github.com/cdot-network/ibc-demo/tree/master/cli) to make the cross-chain work.

## Local Development

Follow these steps to prepare a local Substrate development environment :hammer_and_wrench:

### Setup

Setup instructions can be found at the
[Substrate Developer Hub](https://substrate.dev/docs/en/knowledgebase/getting-started).

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

Open a terminal and run the following command to start a test chain called appia.
```bash
./target/release/node-template --base-path /tmp/chain-appia --dev
```

Open another terminal and start the flaminia test chain.
```bash
./target/release/node-template --base-path /tmp/chain-flaminia --dev --port 20333 --ws-port 8844
```

Create a client of flaminia chain on appia chain, and then create a client of appia chain on flaminia.
```bash
./target/release/cli appia create-client flaminia
./target/release/cli flaminia create-client appia
```

Bind ports for two chains.
```bash
./target/release/cli appia bind-port bank
./target/release/cli flaminia bind-port bank
# ./target/release/cli appia release-port bank // don't
```

Open a new terminal and run relayer.
```bash
export RUST_LOG=relayer=info
./target/release/relayer -c relayer/config.toml
```

Use the cli tool to initiate a connection.
The 2 client IDs below "53a9...fa10 779c...8e03" are from "relayer/config.toml"
```bash
./target/release/cli appia conn-open-init 53a954d6a7b1c595e025226e5f2a1782fdea30cd8b0d207ed4cdb040af3bfa10 779ca65108d1d515c3e4bc2e9f6d2f90e27b33b147864d1cd422d9f92ce08e03
```

When the log shows that the connection status of both chains is open, initiate a channel handshake.
The connection ID below "d93f...bd11" is from the stdout of the command above
```bash
./target/release/cli appia chan-open-init d93fc49e1b2087234a1e2fc204b500da5d16874e631e761bdab932b37907bd11 bank bank
```

When the log shows that the channel status of both chains is open, you can send cross-chain messages as the following command.
The 2 channel IDs below "00e2...86ac a161...601e" are from stdout of the command above, "01020304" is the data to send by channel, in Hex format.
```bash
./target/release/cli appia send-packet 1 1000 bank 00e2e14470ed9a017f586dfe6b76bb0871a8c91c3151778de110db3dfcc286ac bank a1611bcd0ba368e921b1bd3eb4aa66534429b14837725e8cef28182c25db601e 01020304
```
After some blocks, you can see that the flamenia log shows that the packet has been received.

### How the Demo Commands Implemented in Source Code
* In cli, substrate-subxt invokes the pallet's callable functions by the macro ```substrate_subxt_proc_macro::Call```. Please refer to document [substrate_subxt_proc_macro::Call](https://docs.rs/substrate-subxt-proc-macro/0.12.0/substrate_subxt_proc_macro/derive.Call.html) for details.

#### Creating a Client
```
USAGE:
    cli <CHAIN> create-client <chain-name>
```

After the command is triggered, the following functions are executed in sequence.

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/cli/src/main.rs
async fn create_client(
    ...
) -> Result<(), Box<dyn Error>> {
    ...
}
```

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/pallets/template/src/lib.rs
pub fn test_create_client(
    ...
) -> dispatch::DispatchResult {
    ...
}
```

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/pallets/ibc/src/lib.rs
pub fn create_client(
	...
) -> dispatch::DispatchResult {
	...
}	

```

#### Binding a Port
```
USAGE:
    cli <CHAIN> bind-port <identifier>
```

After the command is triggered, the following functions are executed in sequence.

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/cli/src/main.rs
async fn bind_port(addr: &str, identifier: Vec<u8>) -> Result<(), Box<dyn Error>> {
    ...
}
```

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/pallets/template/src/lib.rs
pub fn test_bind_port(origin, identifier: Vec<u8>) -> dispatch::DispatchResult {
    ...
}
```

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/pallets/ibc/src/lib.rs
pub fn bind_port(identifier: Vec<u8>, module_index: u8) -> dispatch::DispatchResult {
	...
}
```

#### Relaying
```
USAGE:
    relayer --config <FILE>
```

After the command is triggered, the following function keep scanning the 2 chains(A & B) for the jobs:
* Block header synchronization
* Connection handshakes
* Channel handshakes
* Packet flow

```rust
async fn relay(
    ...
) -> Result<(), Box<dyn Error>> {
    ...

    // Block header synchronization
    // For the 2 parties on inter-blockchain communication, if one chain(counterparty_client) doesn't have latest block header of the other chain(client).
    if counterparty_client_state.latest_height < block_number {
        ...
    }
    ...
    
    // Scanning the 2 chains' connection state
    // If it detects any party of the 2 can move connection opening handshakes state forward, it sends a request to the corresponding party. 
    for connection in client_state.connections.iter() {
        ...
        // If current handshake state is (A,B)->(INIT, none), it sends a request to chain B, which converts the handshake state to (INIT, TRYOPEN)  
        if connection_end.state == ConnectionState::Init
            && remote_connection_end.state == ConnectionState::None {
            ...    
        }
        // If current handshake state is (A,B)->(INIT, TRYOPEN), it sends a request to chain B, which converts the handshake state to (OPEN, TRYOPEN)  
        else if connection_end.state == ConnectionState::TryOpen
            && remote_connection_end.state == ConnectionState::Init
        {
            ...
        }
        // If current handshake state is (A,B)->(OPEN, TRYOPEN), it sends a request to chain B, which converts the handshake state to (OPEN, OPEN)  
        else if connection_end.state == ConnectionState::Open
            && remote_connection_end.state == ConnectionState::TryOpen {
            ...    
        }
    }

    ...

    // Scanning the 2 chains' channel state
    // If it detects any party of the 2 can move channel opening handshakes state forward, it sends a request to the corresponding party.     
    for channel in client_state.channels.iter() {
        ...
        // If current handshake state is (A,B)->(INIT, none), it sends a request to chain B, which converts the handshake state to (INIT, TRYOPEN)  
        if channel_end.state == ChannelState::Init && remote_channel_end.state == ChannelState::None {
            ...    
        }
        // If current handshake state is (A,B)->(INIT, TRYOPEN), it sends a request to chain B, which converts the handshake state to (OPEN, TRYOPEN)  
        else if channel_end.state == ChannelState::TryOpen
            && remote_channel_end.state == ChannelState::Init
        {
            ...
        }
        // If current handshake state is (A,B)->(OPEN, TRYOPEN), it sends a request to chain B, which converts the handshake state to (OPEN, OPEN)  
        else if channel_end.state == ChannelState::Open
            && remote_channel_end.state == ChannelState::TryOpen {
            ...    
        }    
    }

    // Scanning the 2 events in packet flow: RawEvent::SendPacket, RawEvent::RecvPacket
    // If it detects either event from one party(chain A), it sends corresponding request to the other party(chain B) to move the packet flow forward
    for event in events.into_iter() {
        match event.event {
            node_runtime::Event::pallet_ibc(pallet_ibc::RawEvent::SendPacket(  // Detects event RawEvent::SendPacket from one party(chain A), who initializes sending packet 
                ...
            ) {
                ...
                let datagram = Datagram::PacketRecv {
                    ...
                };
                tx.send(datagram).unwrap(); // Sends the packet to the other party(chain B)
            }
            node_runtime::Event::pallet_ibc(pallet_ibc::RawEvent::RecvPacket( // Detects event RawEvent::RecvPacket, an acknowledgement, from the other party(chain B), who acknowledges after receiving the packet 
                ...
            )) => {
                ...
                let datagram = Datagram::PacketAcknowledgement {
                    ...
                }
                };
                tx.send(datagram).unwrap(); // Sends the acknowledgement to chain A
            }
        ...
    }
}
```

#### Opening a Connection
```
USAGE:
    cli <CHAIN> conn-open-init <client-identifier> <counterparty-client-identifier>
```

After the command is triggered, the following functions are executed in sequence.

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/cli/src/main.rs
async fn conn_open_init(
    ...
) -> Result<(), Box<dyn Error>> {
    ...
}
```

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/pallets/template/src/lib.rs
pub fn test_conn_open_init(
    ...
) -> dispatch::DispatchResult {
    ...
}
```

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/pallets/ibc/src/lib.rs
pub fn conn_open_init(
    ...
) -> dispatch::DispatchResult {
	...
}
```

#### Opening a Channel
```
USAGE:
    cli <CHAIN> chan-open-init [FLAGS] <connection-identifier> <port-identifier> <counterparty-port-identifier>
```

After the command is triggered, the following functions are executed in sequence.

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/cli/src/main.rs
async fn chan_open_init(
    ...
) -> Result<(), Box<dyn Error>> {
    ...
}
```

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/pallets/template/src/lib.rs
pub fn test_chan_open_init(
    ...
) -> dispatch::DispatchResult {
    ...
}
```

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/pallets/ibc/src/lib.rs
pub fn chan_open_init(
    ...
) -> dispatch::DispatchResult {
	...
}
```

#### Sending a Packet
```
USAGE:
    cli <CHAIN> send-packet <sequence> <timeout-height> <source-port> <source-channel> <dest-port> <dest-channel> <data>
```

After the command is triggered, the following functions are executed in sequence.

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/cli/src/main.rs
async fn send_packet(
    ...
) -> Result<(), Box<dyn Error>> {
    ...
}
```

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/pallets/template/src/lib.rs
pub fn test_send_packet(
    ...
) -> dispatch::DispatchResult {
    ...
}
```

```rust
// https://github.com/cdot-network/ibc-demo/tree/master/pallets/ibc/src/lib.rs
pub fn send_packet(
    ...
) -> dispatch::DispatchResult {
	...
}
```
