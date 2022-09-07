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
cd provisioner
RUST_LOG=info cargo run -- --token 84783e12f11c4dcd
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

```
export UUID=<uuid> # Same as you used for the microbit
echo '{"device":"<uuid>"}' | http POST https://api.sandbox.drogue.cloud/api/command/v1alpha1/apps/eclipsecon-hackathon/devices/provisioner command==provision "Authorization:Bearer $(drg whoami -t)"
```

## Create device in Drogue Cloud

Make a note of the assigned address from either the provisioner console or the microbit device console, and create (using 00bf as an example address here):

```
drg create device 00bf --application eclipsecon-hackathon
drg set gateway 00bf gateway --app eclipsecon-hackathon
```

## Optional: Running the simulator
