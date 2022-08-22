# eclipsecon-2022-hackathon

*WORK IN PROGRESS*

This repository contains software, firmware and documentation for the EclipseCon 2022 hackathon.

## Bluetooth Mesh Gateway

The `gateway` folder contains a Bluetooth Mesh gateway that can run on any Linux host with bluez, and
forwards mesh model events to a Drogue Cloud instance.

## BBC micro:bit firmware

The `firmware` folder contains the micro:bit firmware that runs a Bluetooth Mesh node.

The Bluetooth Mesh models supported by the firmware are defined in [MESHMODEL](MESHMODEL.md).

## BBC micro:bit simulator

There are two simulators: 

* A mesh node simulator that runs on any Linux host with bluez, and simulates the exact same models as
the micro:bit firmware and sends messages via the gateway. 

* A WASM-based simulator that sends messages directly to Drogue IoT, which can run in any browser. This can be used to prototype backend applications without needing to set up a mesh network. Note this this uses HTTP unlike the gateway which uses MQTT.

## Model conversion 

The `model-converter` folder contains a HTTP server which is invoked by Drogue Cloud for each sensor event, and will convert data from the Bluetooth Mesh model format to the JSON format described in [MESHMODEL](MESHMODEL.md).

## Digital twin

Should be there soon
