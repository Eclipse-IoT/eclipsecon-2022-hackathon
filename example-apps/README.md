# Example applications

This repository contains simple java apps that subscribe to the various endpoints and 
consume the data provided by the sensors.
It's designed to be simple, so you can look around and grab the building blocks to 
get going with your own ideas !

- [websocket-client](websocket-client) is the simplest app, it subscribes to all events coming through the hackathon devices and log them on the console.
- [console](console) allows you to claim the device that you were handed out. It uses MQTT to stream
the data coming from the claimed device.
- The [dashboard](dashboard) aggregates the data coming from all the devices, using websocket. It then disaply a live dashboard.

# Consuming sensor data

There are a few different way an application can obtain the data from a sensor: 
    - subscribe to the event stream through the integration endpoints (MQTT or WebSocket)
    - get the events directly from the kafka topic
    - Query doppelgaenger (our digital kinda twin) to get the last known state of the device

It's worth to note that sensors sends out "partial" updates, meaning that if only one button was pressed, the payload 
will only contain the state of the button. Other sensors data may not be included in each update. 
In order to get the aggregated data, you must query the doppelgaenger.

### Endpoints info

#### WebSocket

The websocket service is a simple endpoint that forward any events for an application into a websocket, as text. 

host: ws-integration.sandbox.drogue.cloud
port: 443

You can find more details on how to set up the connection [here](https://book.drogue.io/drogue-cloud/dev/user-guide/integration-ws.html)

#### MQTT

The MQTT integration service allows to consume events from applications, just as the websocket endpoint, but also to send commands
back to devices.

You can find more details on how to set up the connection [here](https://book.drogue.io/drogue-cloud/dev/user-guide/integration-mqtt.html)

host: mqtt-integration.sandbox.drogue.cloud
port: 443


#### Doppelgaenger
    
The doppelgaenger is our spin on a digital twin service. It consolidates events from devices into a last known complete
state that you can query. While the MQTT and WS services must be consumed in a "stream" fashion, the doppelgaenger is the 
other way around : your application can query it whenever you want. 

The REST API documentation for the doppelgaenger can be found [here](https://api-eclipsecon-2022.apps.sandbox.drogue.world/).

## Authentication

All the endpoints require authentication in order to consume data. 
The username and token will be handed out at the start of the conference.

Note that the doppelgaenger and the MQTT/WS endpoints use differents SSO instances so the credentials will be different.

## Sending commands to the device

Drogue cloud also supports sending commands back to devices. To do so, you can either use the MQTT endpoint
or the dedicated REST command endpoint.

To use the REST endpoint you need to POST to `https://$host/api/command/v1alpha1/apps/{application}/devices/{device}`, 
with a body containing the payload of your command. 

host: https://api.sandbox.drogue.cloud
port: 443

Drogue cloud will forward any json payload to a device.

You can find [here](https://book.drogue.io/drogue-cloud/dev/api/endpoints.html#_command_control) the API spec for the 
HTTP endpoint, and [here](https://book.drogue.io/drogue-cloud/dev/user-guide/integration-mqtt.html#_publish_commands) 
for the MQTT endpoint.

### supported payload

Sending JSON to device is nice, but sending something that will actually do something is better. 
Here are the commands supported by the sensor board : 

```yaml
todo
```
