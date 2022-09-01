# Drogue IoT: Quarkus MQTT integration example

[![CI](https://github.com/drogue-iot/quarkus-mqtt-integration-example/workflows/CI/badge.svg)](https://github.com/drogue-iot/quarkus-mqtt-integration-example/actions?query=workflow%3A%22CI%22)
[![Matrix](https://img.shields.io/matrix/drogue-iot:matrix.org)](https://matrix.to/#/#drogue-iot:matrix.org)

This is an example of using the MQTT integration of [Drogue IoT](https://drogue.io) in combination
with [Quarkus](https://quarkus.io/).

## What does it do?

It is a small Quarkus application, which connects to the MQTT integration endpoint of Drogue IoT. Receiving messages
from micro:bit devices publishing to Drogue IoT Cloud.

The application starts a web-app that displays the current device readings, and allows
you to trigger commands being sent back to the device.

When you press the 'Update' button, the new settings will be sent to the device.

## How can you run it?

This is a normal Quarkus application, and you can simply run it using `mvn quarkus:dev`. Or any of the other ways
Quarkus applications can be run.

The repository has a [`deploy/`](/deploy) folder, which contains deployment scripts for Kubernetes.

All you need do is to fill in the values in the `010-configuration.template.yaml`, and deploy the YAML files.

## What is needed to run this?

A bunch of things come together here. You can decide how much you want to self-host, how much you want to tweak, and how
much you just want to consume:

* A BBC micro:bit v2 _or_ using the [simulator](https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon).
* Run this application, and point it to the MQTT integration of Drogue Cloud
    * As mentioned above, you can run this locally, on your own Kubernetes server, or with Podman, or in some other way
      a Quarkus application can run.
