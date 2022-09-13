# Developing

This guide covers setting up the mesh infrastructure so that you can run the gateway and devices on a linux host.

## Running the mesh daemon

The mesh daemon communicates with the bluetooth adapter and uses the provided config to store information about the nodes in the mesh.

```
# Ensuring that it's not auto-started by some reconcilation
sudo systemctl disable bluetooth
sudo systemctl stop bluetooth

cd meshd
mkdir -p ${PWD}/lib
sudo /usr/libexec/bluetooth/bluetooth-meshd --config ${PWD}/config --storage ${PWD}/lib --debug
```

## Build the project

```
cargo build --release
```

## Starting the gateway

NOTE: Choose a start address between 0x00ab and 0x7fff at random to avoid conflict with others.

```
RUST_LOG=info ./target/release/eclipsecon-gateway --token dd26596e54e78fa2 --provisioner-token 84783e12f11c4dcd --provisioner-start-address 0x0100
```

## Running the microbit

Flash the microbit with the desired UUID which you will use when provisioning via Drogue Cloud. If you wish to use the [console](https://console-eclipsecon-2022.apps.sandbox.drogue.world/), you must choose a UUID present in [idmap.json](https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon/blob/main/console/src/main/resources/META-INF/resources/idmap.json).

NOTE: Make sure the UUID is a 16 byte hex string without the '-' characters!

```
DEVICE_UUID=<uuid> cargo run --release
```

## Provision the device using the command line

To provision the microbit, we create a new device and set the UUID:

```
drg create device mydevice --app eclipsecon-hackathon --spec '{"btmesh":{"device":"<UUID>"},"gatewaySelector":{"matchNames":["gateway1", "gateway2", "gateway3", "gateway4", "gateway5"]}}'
```

The operator will reconcile the state of the device and send the provisioning command to the device.

You can look at the status section of the device to see when it has been successfully provisioned.

## Provision microbit using the console

Go to the [console](https://console-eclipsecon-2022.apps.sandbox.drogue.world/) and use the claim id corresponding to the UUID you chose earlier.

## Optional: Running the simulator


## Troubleshooting

### Device gets provisioned but does not send any events

Make sure your meshd state is reset:

```
# Stop meshd and gateway

sudo git clean -x -f -d meshd

# Start meshd
# Start gateway
```
