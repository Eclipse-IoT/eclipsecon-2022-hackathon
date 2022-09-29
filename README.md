# EclipseCon 2022 – IoT Hackathon

This repository contains software, firmware and documentation for the EclipseCon 2022 IoT hackathon.

If you have questions, please reach out to us before, during or even after the hackathon. Either online, or in-person
during the conference.

We might do a short introduction during the community day on Monday. Or you can ask at the Red Hat booth.

Some important links:

* [Slides](https://docs.google.com/presentation/d/1xq4taSold1GCIjKdR9hy7n3Cyyb4GKXRvnE2stboVw4)
* [Scratchpad](https://hackmd.io/9D5pbxHlR-KH236PSqaYew)
* Applications:
  * [Console](https://console-eclipsecon-2022.apps.sandbox.drogue.world/)
  * [Dashboard]( https://dashboard-eclipsecon-2022.apps.sandbox.drogue.world/)

## Before the hackathon

We will need a laptop with the ability to run some developer tools. That may include Java, Rust, NodeJS, depending on
what you want to explore and work with. Having an editor or IDE with support for those technologies is also recommended.
It may also include other developer tooling such as Maven, Docker, NPM, … or having the ability to install those.

For flashing new firmware to the micro:bit, you will also need to be able to connect USB devices to your laptop.

You will need to create a user in the Keycloak instance we use for single sign-on. You will be directed to this
instance during the login process of the console. You can choose to either manually create an account and select a
password, or use GitHub as identity provider. It is required to enter an e-mail address, however the address is not
checked and will not be used (also not for requesting a new password).

## Claiming your device

If you received a micro:bit at the event, it should have an ID sticker on the back side of the box. Power on the device using the USB cable or the battery connector, go to the [console](https://console-eclipsecon-2022.apps.sandbox.drogue.world/), log in and then enter the ID to claim the device so that it gets created, provisioned and associated with your account. In the console, you can see the status of your device. You can also release the claim on the device so that it can be provisioned again.

Once claimed and while powered, the device will emit sensor readings, battery status and button events, and is also able to respond to commands sent from the console. You can view real time data
from all the devices on the read only [dashboard](https://dashboard-eclipsecon-2022.apps.sandbox.drogue.world/)

Before the hackathon starts, feel free to play around with the [sample applications](example-apps) we have made.

## Hackathon

For the hackathon, an instance of [Drogue IoT](https://www.drogue.io) as well as local Bluetooth Mesh gateways are provided for you to use. The hackathon itself includes example tasks where you can:

* Write firmware for the BBC micro:bit v2 in Rust. The firmware uses Bluetooth Mesh to send and receive data from the cloud applications.
* Write Quarkus applications running locally and on OpenShift. The application consumes telemetry and produces commands using [cloud events](https://cloudevents.io/).

For working on microcontroller firmware, have a look at the [firmware](firmware/) directory for more information and example tasks.

For working on a quarkus application consuming data, have a look at the [example apps](example-apps).

## BBC micro:bit simulator

If you don't have a BBC micro:bit, you can use one of the following ways to simulate a device:

* A WASM-based simulator that connects to Drogue IoT, and which can run in any browser. Use this to prototype backend applications without needing to set up a bluetooth mesh network. Note that this simulates a single device, not a full mesh.
  
  In the "Console" application, you can "claim" a simulator, by clicking on "create simulator" instead of claiming a device. This will set up your account for using a virtual device. It will also provide a link to the simulator, pre-configuring the access credentials.

  The access information will also be shown in the console. You can also navigate manually to the WASM-based simulator by entering https://web-simulator-eclipsecon-2022.apps.sandbox.drogue.world in your browser, configure the parameters and press 'Run'.

  The source for the simulator can be found [here](web-simulator/).

* A bluetooth mesh node simulator that runs on any Linux host with bluez, and transports events via the gateway. This is mostly useful when you're working on the gateway as it requires a gateway to be deployed, but it also allows you to follow the full provisioning lifecycle.


## Sensor data model

The sensor data is wrapped in a [cloudevent](cloudevents.io) (when consuming from the MQTT and WS endpoints).
Here is what the payload of a partial update looks like, excluding the cloudevent metadata :

```json
{
  "partial": true,
  "state": {
    "sensor": {
      "location": 256,
      "payload": {
        "acceleration": {
          "x": 32,
          "y": -92,
          "z": -1028
        },
        "noise": 8,
        "temperature": 29
      }
    }
  }
}
```

Here is a complete schema of the values a device may send:

```yaml
partial: bool # whether the update is partial or complete 
state:
  sensor:
    location: u8 # Location of the element on the device
    payload:
      acceleration: # Accelerometer values
        x: i16
        y: i16
        z: i16
      noise: u8
      temperature: i8 # the temperature, a Celsius value.
  battery: 
    flags: 
      presence: String # possible values are "NotPresent" or "PresentRemovable"
    level: u8 
    location: u8

# todo complete
```