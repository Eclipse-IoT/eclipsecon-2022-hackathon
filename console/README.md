# EclipseCon 2022 Hackathon Console

This is source code for the EclipseCon 2022 Hackathon console which is where participants can claim the handed-out devices and test device connectivity with the default firmware.

## What does it do?

It is a small Quarkus application, which connects to the MQTT integration endpoint of Drogue IoT, the device registry and a database storing device claim status.

Once claimed, the application will display messages received from the microbit.

When you press the 'Update' button, the new settings will be sent to the device.

## How can you run it?

This is a normal Quarkus application, and you can simply run it using `mvn quarkus:dev`. Or any of the other ways
Quarkus applications can be run.

### Locally

This application requires a PostgreSQL and a Keycloak instance. You can start one using:

```shell
podman-compose -f develop/docker-compose.yaml up
```

Then start the Quarkus application from your IDE, or using:

```shell
mvn quarkus:dev
```

### Kubernetes

The repository has a [`deploy/`](/deploy) folder, which contains deployment scripts for Kubernetes.

All you need do is to fill in the values in the `010-configuration.template.yaml`, and deploy the YAML files.

## What is needed to run this?

A bunch of things come together here. You can decide how much you want to self-host, how much you want to tweak, and how
much you just want to consume:

* A BBC micro:bit v2 _or_ using the [simulator](https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon).
* Run this application, and point it to the MQTT integration of Drogue Cloud
    * As mentioned above, you can run this locally, on your own Kubernetes server, or with Podman, or in some other way
      a Quarkus application can run.
