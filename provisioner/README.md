# eclipse-hackathon-provisioner

The `provisioner` is a Bluetooth Mesh provisioner that can run on any Linux host with bluez, and
accepts commands from and emits events to a Drogue Cloud instance.

## Prerequisites

* [Run meshd](../meshd/README.md)

Run the provisioner

```
cargo run --release -- --token 84783e12f11c4dcd
```
