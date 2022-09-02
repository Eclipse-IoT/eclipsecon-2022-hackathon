# eclipsecon-device

Firmware running on the EclipseCon 2022 Hackathon devices. The firmware implements a Bluetooth Mesh end node according to the Bluetooth Mesh specification. The sensors and peripherals of the micro:bit is exposed through standardized mesh models:

* Button 'A' and 'B' - Generic OnOff Client. This model allows a client to emit `on` and `off` events, which can be used to trigger some event or command.
* LED matrix - Generic OnOff Server. An external command can be used to put the server into an `on` or `off` state. On the micro:bit, the `on` state will enable a blinker task, whereas the `off` state will disable that task.
* Builtin temperature - SensorServer. The sensor server can be configured to emit sensor data periodically.
* Battery - Generic Battery Server. The battery server can be configured to emit battery status events periodically.

In addition, there are a few TODO's for hackathon participants who wants to get their hands dirty with embedded Rust. Each task has complementary work on the cloud side for processing data and sending commands.

* Use the accelerometer to implement a motion detection device. The `accelerometer` module can be used to retrieve accelerometer data, and you can extend the sensor server to emit motion detection events.
* Use the microphone to implement a noise detection device. The `microphone` module can be used to detect noise, and you can extend the sensor server implementation to emit noise detection events.
* Use the onboard speaker to implement a jukebox for playing simple tunes. The `speaker` module can be used to apply audio output on the microbit using Rust types representing notes by pitch and length. 

Installing the toolchain software as instructed below before the event allows you to spend more time on the hackathon tasks!

## Prerequisites

Hardware:

* [BBC micro:bit v2](https://microbit.org/)

Software:

* [`rustup`](https://rustup.rs/)
* [`probe-run`](https://github.com/knurling-rs/probe-run)
* (Optional) [`probe-rs-cli`](https://github.com/probe-rs/probe-rs)


## (Optional) Bootstrapping

NOTE: This step is only necessary if you bring your own micro:bit to the event.

Download the [softdevice](https://www.nordicsemi.com/Products/Development-software/S140/Download) and unpack.

Flash the softdevice onto the micro:bit (only needed the first time you run it):

```
probe-rs-cli download s140_nrf52_7.3.0_softdevice.hex --format Hex --chip nRF52833_xxAA
```

## Running the application

To run the application, make sure your device is connected, and run the following command:

```
cargo run --release
```
