<p align="center">
  <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" alt="Ferris the crab" width="180">
</p>

# Mini Ledger Rust gRPC

The mini ledger rust implementation using gRPC

## Prerequisites

- [Protocol Buffers (protoc)](https://github.com/protocolbuffers/protobuf)
- [gRPC](https://grpc.io/)

## Service Definition

The gRPC service is defined in the `proto/service.proto` file:
https://github.com/leomonteiro92/mini_ledger_grpc/blob/main/proto/service.proto

## Running the Server

Start the server
```shell
cargo build && cargo run
```

## Additional Resources

- [gRPC Official Documentation](https://grpc.io/docs/)
- [Protocol Buffers Guide](https://developers.google.com/protocol-buffers)
