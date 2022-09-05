# eclipse-hackathon-gateway

The `gateway` is a Bluetooth Mesh gateway that can run on any Linux host with bluez, and
forwards mesh model events to a Drogue Cloud instance.

## Prerequisites

* Run meshd with preprovisioned devices

```
[burrboard/gateway]$ sudo /usr/libexec/bluetooth/bluetooth-meshd --config ${PWD}/deploy/bluez/example/meshcfg --storage ${PWD}/deploy/bluez/example/mesh --debug
```

## Running

Run the application:

```
cargo run --release -- --token dd26596e54e78fa2
```

Gateway has address `00bc` and subscribs to virtual address `8f32`
