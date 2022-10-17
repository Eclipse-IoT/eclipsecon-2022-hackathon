# pre-provision

Utility based on probe-rs for pre-provisioning microbits.

## Installation

```
cargo install --path .
```

## Usage

```
cargo run -- provision --flash-address=0x0007E000 --node-address 0x0100 --network-key 0B5E6760156116BAB83115D4C1BFB480 --application-key 8E0A245C38A136E7D6E8429D562DA959  --chip nRF52833_xxAA
```
