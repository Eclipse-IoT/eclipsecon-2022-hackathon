# Drogue IoT: Quarkus MQTT integration example

[![CI](https://github.com/drogue-iot/quarkus-mqtt-integration-example/workflows/CI/badge.svg)](https://github.com/drogue-iot/quarkus-mqtt-integration-example/actions?query=workflow%3A%22CI%22)
[![Matrix](https://img.shields.io/matrix/drogue-iot:matrix.org)](https://matrix.to/#/#drogue-iot:matrix.org)

This is an example of using the MQTT integration of [Drogue IoT](https://drogue.io) in combination
with [Quarkus](https://quarkus.io/).

## What does it do?

It is a small Quarkus application, which connects to the MQTT integration endpoint of Drogue IoT. Receiving messages
from devices publishing to Drogue IoT Cloud.

It will parse messages and extract TTN (The Things Network) uplink messages from that stream. When a TTN uplink message
is received for port 1 and with the payload `ping`, then it will respond with the current active response command.

The response will be sent back to the device using a downlink message.

NOTE: A downlink message will only be sent shortly after the uplink message was received, as this is when the device
expects it. Setting a new response command will not send a new downlink message.

When you sent `led:on` to the device, the blue LED should turn on. When you send `led:off`, it should turn off again.

NOTE: There is a short time window, between the device sending and receiving. It may happen that the downlink command
takes a bit too long, and so it is delivered the next time the device connects.

## How can you run it?

This is a normal Quarkus application, and you can simply run it using `mvn quarkus:dev`. Or any of the other ways
Quarkus applications can be run.

The repository has a [`deploy/`](/deploy) folder, which contains deployment scripts for Kubernetes.

All you need do is to fill in the values in the `010-configuration.template.yaml`, and deploy the YAML files.

## What is needed to run this?

A bunch of things come together here. You can decide how much you want to self-host, how much you want to tweak, and how
much you just want to consume:

* An STM32L0 Discovery kit for LoRa, … (B-L072Z-LRWAN1) -> https://www.st.com/en/evaluation-tools/b-l072z-lrwan1.html
    * Yes, you can use a different board, but you might need to tweak the firmware.
* TTN coverage
    * Maybe you need your own LoRaWAN gateway, connected to TTN
* The firmware
  from [drogue-iot/drogue-device-ng](https://github.com/drogue-iot/drogue-device-ng/tree/main/examples/stm32l0xx/lora-discovery)
* A TTN (The Things Network) account (for v3 of the API)
    * Including an API key which allows to create applications and devices
* A Drogue IoT that is publicly reachable
    * You can self-host this
    * You can use the "devbox" instance (which is unstable and insecure)
      -> https://console-drogue-dev.apps.wonderful.iot-playground.org/
    * You can use the "sandbox" instance (which is more stable and more secure) -> https://sandbox.drogue.cloud
        * Currently, it is missing the features which this application requires. You need to wait until version 0.5 of
          Drogue Cloud is released.
* Set up the TTN integration of Drogue Cloud -> https://book.drogue.io/drogue-cloud/dev/ttn/index.html
    * Alternatively, you can manually set up the devices and connections.
    * Set up a new application
    * Register a new device
* Run this application, and point it to the MQTT integration of Drogue Cloud
    * As mentioned above, you can run this locally, on your own Kubernetes server, or with Podman, or in some other way
      a Quarkus application can run.
