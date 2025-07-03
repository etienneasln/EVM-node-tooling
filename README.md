# EVM-node-tooling

EVM-node-tooling is a Rust library for the interaction of the EVM node with its database in Etherlink.


## Setup

1. Run the evm node (from [tezos](https://gitlab.com/tezos/tezos)) for a while:
   ```
   ./octez-evm-node run observer   --network mainnet   --history rolling:1   --dont-track-rollup-node   --init-from-snapshot --data-dir ./local_test --profiling
   ```  
2. Run the following command in the repository's root directory:
   ```
   echo DATABASE_URL=/path/to/the/store.sqlite/file > .env
   ```
   The `store.sqlite` is created by the node and located in the node's data directory.  
3. Install Diesel CLI if not already installed (see [Getting Started with Diesel](https://diesel.rs/guides/getting-started))
4. Run `diesel setup` in the root directory

## Running

1. Run the command `cargo run --bin server` in the root directory of the repository.  
2. From a separate terminal, run some HTTP requests:
   ```
   curl -X POST http://localhost:8080/ -H "Content-Type: application/json" --data '{"name":"method_name","params":[#Insert adequate parameters]}'
   ```

## Benchmarks

Note: It is possible to benchmark with an in memory database by changing the `DATABASE_URL` to `:memory:` (not for manual selecting benchmark).

### Criterion 

Run the `cargo bench` command. The benchmarks' execution will depend on and update the database's content. Change the hard-coded values accordingly (`INSERT_INDEX` and `SELECT_INDEX`). 

### Manual Benchmarks

#### Select

Run the `cargo run --bin manualbenchingselectwithlevel` command.

#### Insert

Run the `cargo run --bin manualbenchinginsert` command.