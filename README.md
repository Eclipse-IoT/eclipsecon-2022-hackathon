# eclipsecon-2022-hackathon

This repository contains software, firmware and documentation for the EclipseCon 2022 hackathon.

For the hackathon, an instance of [Drogue IoT](https://www.drogue.io) as well as local Bluetooth Mesh gateways are provided for you to use. The hackathon itself includes example tasks where you can:

* Write firmware for the BBC micro:bit v2 in Rust. The firmware uses Bluetooth Mesh to send and receive data from the cloud applications.
* Write Quarkus applications running locally and on OpenShift. The application consumes telemetry and produces commands using [cloud events](https://cloudevents.io/).

For working on microcontroller firmare, have a look at the [firmware](firmware/) directory for more information and example tasks.

For working on the quarkus application, have a look at the [example apps](example-apps/).

## BBC micro:bit simulator

If you don't have a BBC micro:bit, you can also use on of the following simulators:

* A WASM-based simulator that sends messages directly to Drogue IoT, which can run in any browser. This can be used to prototype backend applications without needing to set up a mesh network. Note this this uses HTTP unlike the gateway which uses MQTT. You can try the WASM-based simulator by entering https://web-simulator-eclipsecon-2022.apps.sandbox.drogue.world in your browser, configure the parameters and press 'Run'.

* A mesh node simulator that runs on any Linux host with bluez, and simulates the exact same models as the micro:bit firmware and sends messages via the gateway. This is mostly useful when you're working on the gateway.

