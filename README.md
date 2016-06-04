## Development

### Protobuf
Concord uses protobuf for exchanging RPC messages.

1. Install protobuf library (ex mac os): `brew install protobuf`
1. Install the rust [plugin](https://github.com/stepancheg/rust-protobuf/): `cargo install protobuf`
1. Add the plugin to your path: `PATH="$HOME/.cargo/bin:$PATH"`
1. Generate: `protoc --rust_out src/raft src/raft/types.proto`
