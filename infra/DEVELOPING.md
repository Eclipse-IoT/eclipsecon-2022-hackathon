# Developing

This guide covers setting up the mesh infrastructure so that you can run the gateway and devices on a linux host.

## Running the mesh daemon

The mesh daemon communicates with the bluetooth adapter and uses the provided config to store information about the nodes in the mesh.

```
# Ensuring that it's not auto-started by some reconcilation
sudo systemctl disable bluetooth
sudo systemctl stop bluetooth

cd infra/meshd
sudo /usr/libexec/bluetooth/bluetooth-meshd --config ${PWD}/config --storage ${PWD}/lib --debug
```
If you don't have `/usr/libexec/bluetooth/bluetooth-meshd` on your system then
you'll need to install the `bluez-mesh` package. For example, to install using
the `dnf` package manager the following command can be used:
```console
sudo dnf install -y bluez-mesh
```

## Build the project

The development of gateway (and simulator) projects is possible only on Linux platforms with `libdbus` installed.
Uncomment appropriate modules in [infra/Cargo.toml](infra/Cargo.toml) to make them a part of the workspace.

```
cargo build --release
```

## Starting the gateway

Ensure the [default Drogue Cloud connection settings](https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon/blob/main/infra/gateway/src/main.rs) match your environment!

```
RUST_LOG=info ./target/release/eclipsecon-gateway --drogue-device gateway1 --drogue-application eclipsecon-hackathon --token dd26596e54e78fa2
```

Or, to build and run:

```shell
cd infra/gateway
RUST_LOG=info cargo run --package eclipsecon-gateway -- --drogue-device gateway1 --drogue-application eclipsecon-hackathon --token dd26596e54e78fa2
```

NOTE: If gateway fails to connect to the cloud MQTT endpoint, check the path to the file containing CA Root certificates and adjust it, e.g. `--ca-path /etc/ssl/certs/ca-certificates.crt`

## Start containerized mesh and gateway

You can use docker compose to easily start both mesh daemon and gateway together

```
cd infra/gateway
develop/init.sh # only the first time and when you want to reset gateway state
docker-compose -f develop/gateway-compose.yaml up
```

## Start gateway in Kubernetes

If you have installed local kubernetes cluster like MicroShift, you can deploy gateway (along with the mesh) with

```
kubectl create namespace eclipsecon-gateway
kubectl apply -n eclipsecon-gateway -f deployments/gateways/gateway-daemonset.yaml
```

## Installing softdevice on microbit (only needed first time)

Download the [softdevice](https://www.nordicsemi.com/Products/Development-software/S140/Download) and unpack.

Flash the softdevice onto the micro:bit (only needed the first time you run it):

```
probe-rs-cli erase --chip nRF52833_xxAA
probe-rs-cli download s140_nrf52_7.3.0_softdevice.hex --format Hex --chip nRF52833_xxAA
```

You can also use the `firmware/flashsd.sh` script for that.

## Provisioning the microbit (only needed first time)

Choose a valid unicast network address for your device. If you wish to use the [console](https://console-eclipsecon-2022.apps.sandbox.drogue.world/) to claim the device, you must choose an address present in [idmap.json](https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon/blob/main/example-apps/console/src/main/resources/META-INF/resources/idmap.json).

NOTE: Make sure the address is a 2 byte hex string with the '0x' prefix, and
use the `devkey` value with the `--device-key` option in the following commands:

```
cd infra/pre-provision
cargo run -- provision --flash-address=0x7E000 --node-address 0x100 --network-key 0B5E6760156116BAB83115D4C1BFB480 --application-key 8E0A245C38A136E7D6E8429D562DA959 --device-key 505f1bd9bd5b58418e964b0fcd2aaf88 --chip nRF52833_xxAA
cargo run -- provision --flash-address=0x7F000 --node-address 0x100 --network-key 0B5E6760156116BAB83115D4C1BFB480 --application-key 8E0A245C38A136E7D6E8429D562DA959 --device-key 505f1bd9bd5b58418e964b0fcd2aaf88 --chip nRF52833_xxAA
```
```

## Create the device in Drogue Cloud

Use the same address as pre-provisioned when creating the device

```
drg login https://api.sandbox.drogue.cloud
drg create device mydevice --application eclipsecon-hackathon --spec '{"alias":["0100"], "gatewaySelector":{"matchNames":["gateway1","gateway2","gateway3","gateway4","gateway5"]}}'
drg label device mydevice --application eclipsecon-hackathon role=node
```

## Running the microbit

To start the device, flash the firmware:

```
cd firmware
cargo run --release
```

## Optional: Running the simulator

If you don't have a microbit, you can run the simulator:

```
cd infra/simulator
RUST_LOG=info ./target/release/eclipsecon-simulator --device <uuid>
```


## Claim the device using the console

Go to the [console](https://console-eclipsecon-2022.apps.sandbox.drogue.world/) and use the claim id corresponding to the address you chose for the device.

## Troubleshooting

### Device gets provisioned but does not send any events

Make sure your meshd state is reset:

```
# Stop meshd and gateway

sudo git clean -x -f -d meshd

# Start meshd
# Start gateway
```
