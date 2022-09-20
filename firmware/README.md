# eclipsecon-device

Firmware running on the EclipseCon 2022 Hackathon devices. The firmware implements a Bluetooth Mesh end node according to the Bluetooth Mesh specification. The sensors and peripherals of the micro:bit is exposed through standardized mesh models:

* Button 'A' and 'B' - Generic OnOff Client. This model allows a client to emit `on` and `off` events, which can be used to trigger some event or command.
* LED matrix - Generic OnOff Server. An external command can be used to put the server into an `on` or `off` state. On the micro:bit, the `on` state will enable a blinker task, whereas the `off` state will disable that task.
* Builtin temperature - SensorServer. The sensor server can be configured to emit sensor data periodically.
* Battery - Generic Battery Server. The battery server can be configured to emit battery status events periodically.

In addition, there are a few TODO's for hackathon participants who want to get their hands dirty with embedded Rust. Each task has complementary work on the cloud side for processing data and sending commands.

* Use the accelerometer to supply x, y, z acceleration data. The `sensor` module can be modified used to read accelerometer data and emit motion 3D acceleration readings.
* Use the microphone to implement a noise detection device. The `sensor` module can be modified to read sound levels and emit sound level values.
* Use the onboard speaker to implement a jukebox for playing simple tunes. Modify the `onoff` module to play a tune together with the existing blinking when 'on'.

Installing the toolchain software as instructed below before the event allows you to spend more time on the hackathon tasks!

## Prerequisites

Hardware:

* [BBC micro:bit v2](https://microbit.org/)

Software:

* [`rustup`](https://rustup.rs/)
* [`probe-run`](https://crates.io/crates/probe-run)
* [`cargo-flash`](https://crates.io/crates/cargo-flash)
* (Optional) [`probe-rs-cli`](https://crates.io/crates/probe-rs-cli)

## (Optional) Bootstrapping

NOTE: This step is only necessary if you bring your own micro:bit to the event.

Download the [softdevice](https://www.nordicsemi.com/Products/Development-software/S140/Download) and unpack.

Flash the softdevice onto the micro:bit (only needed the first time you run it):

```
probe-rs-cli download s140_nrf52_7.3.0_softdevice.hex --format Hex --chip nRF52833_xxAA
```

## Debugging the application

To run the application with debugging attached, make sure your device is connected, and run the following command:

```
DEVICE_UUID=<UUID> cargo run --release
```

## Flashing application for battery powered use

To run off battery, the application should be flashed without the debug probe attach. You can do this by running the following command:

```
DEVICE_UUID=<UUID> cargo flash --release --no-default-features --features panic-reset --chip nRF52833_xxAA
```
