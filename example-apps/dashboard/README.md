# EclipseCon 2022 Hackathon Dashboard

This is source code for the EclipseCon 2022 Hackathon dashboard, which is used to aggregate all device data into a "big screen".

## What does it do?

The backend application connects to the Doppelgaenger, digital twin, instance and scans for devices. Once they show up, it subscribes to their state and uses it to build an aggregated overview.

The overview is rendered on the server side, using Quarkus' template engine, and the outcome
is sent to the frontend through a websocket connection. The frontend simply shows the pre-rendered state.

## Running

You need to provide some configuration, we recommend to create a `.env` file in this directory, providing the following information:

```shell
QUARKUS_OIDC_CLIENT_AUTH_SERVER_URL=https://sso-eclipsecon-2022.apps.sandbox.drogue.world/realms/doppelgaenger
QUARKUS_OIDC_CLIENT_CLIENT_ID=services
QUARKUS_OIDC_CLIENT_CREDENTIALS_SECRET=<service account access token>
```

Then, you can start it like any other Quarkus application. For example:

```shell
mvn quarkus:dev
```
