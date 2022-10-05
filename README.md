# EclipseCon 2022 – IoT Hackathon

This repository contains software, firmware and documentation for the EclipseCon 2022 IoT hackathon.

If you have questions, please reach out to us before, during or even after the hackathon. Either online, or in-person
during the conference.

We might do a short introduction during the community day on Monday. Or you can ask at the Red Hat booth.

Some important links:

* [Slides](https://docs.google.com/presentation/d/1xq4taSold1GCIjKdR9hy7n3Cyyb4GKXRvnE2stboVw4)
* [Scratchpad](https://hackmd.io/9D5pbxHlR-KH236PSqaYew)
* Applications
  * [Console](https://console-eclipsecon-2022.apps.sandbox.drogue.world/)
  * [Dashboard](https://dashboard-eclipsecon-2022.apps.sandbox.drogue.world/)
  * [OPC UA Server](opc.tcp://a619b000489564b4c8d667f2707cfc9f-1586875243.eu-west-1.elb.amazonaws.com:4840/drogue-iot)

## Before the hackathon

You will need a laptop with the ability to run some developer tools. That may include Java, Rust, NodeJS, depending on
what you want to explore and work with. Having an editor or IDE with support for those technologies is also recommended.
It may also include other developer tooling such as Maven, Docker, NPM, … or having the ability to install those.

For flashing new firmware to the micro:bit, you will also need to be able to connect USB devices to your laptop.

You will need to create a user in the Keycloak instance we use for single sign-on. You will be directed to this
instance during the login process of the console. You can choose to either manually create an account and select a
password, or use GitHub as identity provider. It is required to enter an e-mail address, however the address is not
checked and will not be used (also not for requesting a new password).

## Claiming your device

If you received a micro:bit at the event, it should have an ID sticker on the back side of the box. Power on the device using the USB cable or the battery connector, go to the [console](https://console-eclipsecon-2022.apps.sandbox.drogue.world/), log in and then enter the ID to claim the device so that it gets associated with your account. In the console, you can see the status of your device. You can also release the claim on the device so that it can be claimed again.

Once claimed and while powered, the sensor readings, battery status and button events will appear in the console, and it will respond to commands sent from the console. You can view real time data
from all the devices at the event on the read only [dashboard](https://dashboard-eclipsecon-2022.apps.sandbox.drogue.world/)

Before the hackathon starts, feel free to play around with the [sample applications](example-apps) we have made.

## Hackathon

For the hackathon, an instance of [Drogue IoT](https://www.drogue.io) as well as local Bluetooth Mesh gateways are provided for you to use. The hackathon itself includes example tasks where you can:

* Write firmware for the BBC micro:bit v2 in Rust. The firmware uses Bluetooth Mesh to send and receive data from the cloud applications.
* Write Quarkus applications running locally and on OpenShift. The application consumes telemetry and produces commands using [cloud events](https://cloudevents.io/).

For working on microcontroller firmware, have a look at the [firmware](firmware/) directory for more information and example tasks.

For working on a quarkus application consuming data, have a look at the [example apps](example-apps).

## BBC micro:bit simulator

If you don't have a BBC micro:bit, you can use one of the following ways to simulate a device using a WASM-based simulator that connects to Drogue IoT, and which can run in any browser. Use this to prototype backend applications without a device. Note that this simulates a single device, not a full mesh. 

In the "Console" application, you can "claim" a simulator, by clicking on "create simulator" instead of claiming a device. This will set up your account for using a virtual device. It will also provide a link to the simulator, pre-configuring the access credentials.

The access information will also be shown in the console. You can also navigate manually to the WASM-based simulator by entering https://web-simulator-eclipsecon-2022.apps.sandbox.drogue.world in your browser, configure the parameters and press 'Run'.

The source for the simulator can be found [here](web-simulator/).

## Data model

The sensor data is wrapped in a [cloudevent](cloudevents.io) (when consuming from the MQTT and WS endpoints).


### Telemetry (device to cloud)

Here is what the payload of a sensor update looks like, excluding the cloudevent metadata:

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
      presence: String # possible values are "NotPresent", "PresentRemovable", "PresentNotRemovable" or "Unknown"
    level: u8 # A number between 0 and 100 representing the battery level.
    location: u8
  button:
    on: bool
    location: u8 # Use this to distinguish which button is sending the events
```

NOTE: when `partial` is `true`, only a fraction of the state, such as only the `sensor` field may be present.

### Commands (cloud to device)

For sending commands back, the JSON payload should follow this schema, where one of `speaker` or `display` should be set:

```yaml
  address: u16 # Bluetooth Mesh address of the device that the command is sent to. You can find this in the console UI.
  speaker:
    on: bool # Enabling the speaker (NOTE: Requires modifying the firmware)
  display:
    level: u8 # A number between 0 and 10 describing the brightness
```
