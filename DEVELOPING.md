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


## Starting the provisioner

```
# Make sure you pick a start address that doesn't conflict with others in the same mesh
cd provisioner
RUST_LOG=info cargo run -- --token 84783e12f11c4dcd --start-address 0x00c0
```

## Starting the gateway

```
cd gateway
RUST_LOG=info cargo run -- --token dd26596e54e78fa2
```


## Running the microbit

Flash the microbit with the desired UUID which you will use when provisioning via Drogue Cloud.

```
DEVICE_UUID=<uuid> cargo run --release
```

## Provision microbit

To provision the microbit, we create a new device and set the UUID:

```
drg create device mydevice --app eclipsecon-hackathon --spec '{"btmesh":{"device":"<UUID>"},"gatewaySelector":{"matchNames":["provisioner", "gateway1", "gateway2", "gateway3", "gateway4", "gateway5"]}}'
```

The operator will reconcile the state of the device and send the provisioning command to the device.

You can look at the status section of the device to see when it has been successfully provisioned.

## Optional: Running the simulator
