### Game

#### Setup
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. `cargo run` (initial compile will take a long time, Bevy is a large library)


#### Bevy

Bevy reference: https://bevy.org/learn/quick-start/introduction/

#### Networking

Minimal client and server echo demo setup,
1. In terminal 1, run `cargo run --bin server`
2. In terminal 2, run `cargo run --bin client`
3. In terminal 3, run `cargo run --bin client`
4. Send messages from either client, and see the echo from the server, and view the messages received from the server terminal.
