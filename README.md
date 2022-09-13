# eclipsecon-2022-hackathon

This repository contains software, firmware and documentation for the EclipseCon 2022 hackathon.

## Claiming your device

If you received a micro:bit at the event, it should have an ID sticker on the back side of the box. Power on the device using the USB cable or the battery connector, go to the [console](https://console-eclipsecon-2022.apps.sandbox.drogue.world/), log in and then enter the ID to claim the device so that it gets created, provisioned and associated with your account. In the console, you can see the status of your device. You can also release the claim on the device so that it can be provisioned again.

Once claimed and while powered, the device will emit sensor readings, battery status and button events, and is also able to respond to commands sent from the console. You can view real time data
from all the devices on the read only [dashboard](https://dashboard-eclipsecon-2022.apps.sandbox.drogue.world/)

Before the hackathon starts, feel free to play around with a sample applications we have made:

* [Console](https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon/tree/main/console)
* [Dashboard](https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon/tree/main/dashboard)

## Hackathon

For the hackathon, an instance of [Drogue IoT](https://www.drogue.io) as well as local Bluetooth Mesh gateways are provided for you to use. The hackathon itself includes example tasks where you can:

* Write firmware for the BBC micro:bit v2 in Rust. The firmware uses Bluetooth Mesh to send and receive data from the cloud applications.
* Write Quarkus applications running locally and on OpenShift. The application consumes telemetry and produces commands using [cloud events](https://cloudevents.io/).

For working on microcontroller firmare, have a look at the [firmware](firmware/) directory for more information and example tasks.

For working on the quarkus application, have a look at the [example apps](example-apps/).

## BBC micro:bit simulator

If you don't have a BBC micro:bit, you can use on of the following ways to simulate a device:

* A WASM-based simulator that connects to Drogue IoT, and which can run in any browser. Use this to prototype backend applications without needing to set up a bluetooth mesh network. Note this this uses HTTP unlike the gateway which uses MQTT. You can try the WASM-based simulator by entering https://web-simulator-eclipsecon-2022.apps.sandbox.drogue.world in your browser, configure the parameters and press 'Run'. The source for the simulator can be found [here](web-simulator/).

* A bluetooth mesh node simulator that runs on any Linux host with bluez, and transports events via the gateway. This is mostly useful when you're working on the gateway as it requires a gateway to be deployed, but it also allows you to follow the full provisioning lifecycle.

