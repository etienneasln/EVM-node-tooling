# EVM-node-tooling

EVM-node-tooling is a Rust library for the interaction of the EVM node with its database in Etherlink.


## Setup

1.Run the evm node for a while in a separate directory: ```octez-evm-node --network=testnet ...```
2.Copy the ``store.sqlite`` file created following this execution and paste it in the repository's root directory.
3.Modify the ``.env`` file to match the SQLite file's location.
