# EVM-node-tooling

EVM-node-tooling is a Rust library for the interaction of the EVM node with its database in Etherlink.


## Setup

1. Run the evm node (from [tezos](https://gitlab.com/tezos/tezos)) for a while:
   ```
   ./octez-evm-node run observer   --network mainnet   --history rolling:1   --dont-track-rollup-node   --init-from-snapshot --data-dir ./local_test --profiling
   ```  
2. Run the following command in the repository's root directory to make the DATABASE_URL environment variable point to the store.sqlite file. If this step isn't done then an empty `store.sqlite` database (with no tables) will be created in the repository's root directory when the first tests or benchmarks are ran:
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

### Criterion 

Run the `cargo bench` command. Here's a list of custom options with examples:  

- Set a custom block number with the `BLOCK_NUMBER` environment variable. If not set, it will take the top block level in the database.
- Set a custom database path with the `DATABASE_URL` environment variable. If not set, it will take the provided environment variable in the .env file.
- Use the `cargo bench -- <filter>` command-line option provided by Criterion to filter the desired benchmarks with regular expressions. By default, all benchmarks will be ran.

#### Examples
```
BLOCK_NUMBER=200000 DATABASE_URL="./store.sqlite" cargo bench -- "Apply blueprint" 
cargo bench -- "Insert .*"  
BLOCK_NUMBER=200000 cargo bench -- "Apply blueprint"  
```

### Manual Benchmarks

#### Apply Blueprint

Run the `cargo run --bin manualbenchingapplyblueprint` command.


## Testing

Run tests with `cargo test -- --test-threads=1`. Tests have to be ran sequentially because SQLite doesn't allow concurrent writes.