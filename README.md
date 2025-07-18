# EVM-node-tooling

EVM-node-tooling is a Rust library for the interaction of the EVM node with its database in Etherlink.


## Setup

1. Run the evm node (from [tezos](https://gitlab.com/tezos/tezos)) for a while:
   ```
   ./octez-evm-node run observer   --network mainnet   --history rolling:1   --dont-track-rollup-node   --init-from-snapshot --data-dir ./local_test --profiling
   ```  
2. (optional) Run the following command in the repository's root directory to provide a default DATABASE_URL environment variable pointing to the store.sqlite file. If this step isn't done, then you will have to provide the DATABASE_URL environment variable when running binaries (including benchmarks and tests). If no path to the database is provided, then the binaries will panic:
   ```
   echo DATABASE_URL=/path/to/the/store.sqlite/file > .env
   ```
   The `store.sqlite` is created by the node and located in the node's data directory.  

## Running

1. Run the command `cargo run --bin server` in the root directory of the repository.  
2. From a separate terminal, run some HTTP requests:
   ```
   curl -X POST http://localhost:8080/ -H "Content-Type: application/json" --data '{"name":"method_name","params":[#Insert adequate parameters]}'
   ```

## Benchmarks-Apply blueprint

It is possible to benchmark part of the execution of apply_blueprint with this library, namely all the SQL queries and the start/commit of the SQL transaction to the `store.sqlite` file. This is done by running some queries for a specific block number, by generating new hashes for each insert (inserting in the same tables as when the node is running). The only query that can't be benchmarked is the queries in `pending_confirmations` because the table is empty as the node isn't running. It is not possible to run benchmarks pointing to the node's store while the node is running because SQLite does not allow concurrent writes.

### Criterion 

Run the `cargo bench` command. Here's a list of custom options with examples:  

- Set a custom block number with the `BLOCK_NUMBER` environment variable. If not set, it will take the top block level in the database.
- Set a custom database path with the `DATABASE_URL` environment variable. If not set, it will take the provided environment variable in the .env file.
- Use the `cargo bench -- <filter>` command-line option provided by Criterion to filter the desired benchmarks with regular expressions. By default, all benchmarks will be ran. Set the filter to `step` to run the benchmarks for all the queries ran during the application of a blueprint (excluding the execution time for writing out to disk). Set it to `Apply blueprint` to run the whole blueprint application benchmark.

#### Examples
```
$ BLOCK_NUMBER=200000 DATABASE_URL="./store.sqlite" cargo bench -- "Apply blueprint" 
$ cargo bench -- "step"  
$ BLOCK_NUMBER=200000 cargo bench -- "Apply blueprint"  
```

### Manual Benchmarks

#### Apply Blueprint

Run the `cargo run --bin manualbenchingapplyblueprint` command.


## Testing

Run tests with `cargo test -- --test-threads=1`. Tests have to be ran sequentially because SQLite doesn't allow concurrent writes.