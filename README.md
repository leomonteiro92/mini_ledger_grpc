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
```proto
syntax = "proto3";

package service;

import "google/protobuf/timestamp.proto";

service MiniLedger {
    rpc CreateAccount(AccountCreationRequest) returns (AccountResponse);
    rpc Deposit(DepositRequest) returns (TransactionsResponse);
    rpc GetAccount(GetAccountRequest) returns (AccountResponse);
    rpc Transfer(TransferRequest) returns (TransactionsResponse);
    rpc Withdraw(WithdrawRequest) returns (TransactionsResponse);
}

message AccountCreationRequest {
    string uuid = 1;
    string currency = 2;
}

message GetAccountRequest {
    string uuid = 1;
}

message AccountResponse {
    double balance = 1;
    google.protobuf.Timestamp created_at = 2;
    google.protobuf.Timestamp last_updated_at = 3;
    string currency = 4;
    string uuid = 5;
    string version = 6;
}

message DepositRequest {
    double amount = 1;
    string account_uuid = 2;
    string idempotency_key = 3;
}

message WithdrawRequest {
    double amount = 1;
    string account_uuid = 2;
    string idempotency_key = 3;
}

message TransferRequest {
    double amount = 1;
    string from_account_uuid = 2;
    string idempotency_key = 3;
    string to_account_uuid = 4;
}

message TransactionResponse {
    double amount = 1;
    google.protobuf.Timestamp created_at = 2;
    string account_id = 3;
    string account_version = 4;
    string currency = 5;
    string id = 6;
    string idempotency_key = 7;
}

message TransactionsResponse {
    repeated TransactionResponse transactions = 1;
}
```

## Running the Server

Start the server
```shell
cargo build && cargo run
```

## Additional Resources

- [gRPC Official Documentation](https://grpc.io/docs/)
- [Protocol Buffers Guide](https://developers.google.com/protocol-buffers)