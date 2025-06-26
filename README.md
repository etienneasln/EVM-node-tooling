# EVM-node-tooling

EVM-node-tooling is a Rust library for the interaction of the EVM node with its database in Etherlink.


## Setup

1.Run the evm node (from https://gitlab.com/tezos/tezos) for a while in a separate directory: 
```
octez-evm-node --network=testnet ...
```  
2.Modify the ``.env`` file to match the SQLite ``store.sqlite`` file's location. This file is created by the node and located in the node's data directory.
3.(optional) Modify hard-coded values in the tests in ``src/dieselsqlite/models.rs``to reflect the bottom and top block id in the database.  

## Running

1. Run the command ``cargo run --bin server`` in the root directory of the repository.  
2. From a separate terminal, run some HTTP requests:
   ```
   curl -X POST http://localhost:8080/ -H "Content-Type: application/json" --data '{"name":"method_name","params":[#Insert adequate parameters]}'
   ```
