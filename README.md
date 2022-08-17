# eclipsecon-2022-hackathon

*WORK IN PROGRESS*

This repository contains software, firmware and documentation for the EclipseCon 2022 hackathon.

## Bluetooth Mesh Gateway

The `gateway` folder contains a Bluetooth Mesh gateway that can run on any Linux host with bluez, and
is able to forward mesh model events from the microbit models to a Drogue Cloud instance.

## BBC micro:bit Bluetooth Mesh node

The `firmware` folder contains the micro:bit firmware that runs a Bluetooth Mesh node.

The Bluetooth Mesh models supported by the firmware are defined in [MESHMODEL](MESHMODEL.md).

## BBC micro:bit simulator

The micro:bit simulator can run on any Linux host with bluez, and simulates the exact same models as
the micro:bit firmware. This can be used to prototype backend applications without running a
micro:bit.

