# eclipse-hackathon-gateway

The `gateway` is a Bluetooth Mesh gateway that can run on any Linux host with bluez, and forwards mesh model events to a Drogue Cloud instance. It also receives commands from the cloud and sends them to devices.

See the [docs](../DEVELOPING.md) for how to run the gateway.

# Building new images

Run the following commands from the directory this file is located in:

```shell
podman build  ../.. -f infra/gateway/Dockerfile -t quay.io/eclipsecon-2022/gateway:latest
podman push quay.io/eclipsecon-2022/gateway:latest
```
