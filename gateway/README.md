# eclipse-hackathon-gateway

Default gateway for the hackathon.

## Prerequisites

* Run meshd with preprovisioned devices

```
[burrboard/gateway]$ sudo /usr/libexec/bluetooth/bluetooth-meshd --config ${PWD}/deploy/bluez/example/meshcfg --storage ${PWD}/deploy/bluez/example/mesh --debug
```

## Running

Run the application:

```
cargo run --release -- --token 7eb48c91911361da
```
