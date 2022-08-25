package io.drogue.iot.demo;

import java.nio.charset.StandardCharsets;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;

import org.eclipse.microprofile.reactive.messaging.Channel;
import org.eclipse.microprofile.reactive.messaging.Emitter;
import org.eclipse.microprofile.reactive.messaging.Incoming;
import org.eclipse.microprofile.reactive.messaging.Outgoing;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.demo.data.DeviceCommand;
import io.drogue.iot.demo.data.DeviceEvent;
import io.quarkus.runtime.Startup;
import io.smallrye.reactive.messaging.annotations.Broadcast;

/**
 * Process device events.
 * <p>
 * This is main logic in this application. Processing happens in the {@link #process(DeviceEvent)} method.
 * <p>
 * It receives messages from the Drogue IoT MQTT integration, pre-processed by the {@link
 * io.drogue.iot.demo.integration.Receiver}. It can return {@code null} to do nothing, or a {@link DeviceCommand} to
 * send a response back to the device.
 * <p>
 * As this targets a LoRaWAN use case, where the device sends an uplink (device-to-cloud) message, and waits a very
 * short period of time for a downlink (cloud-to-device) message, we must act quickly, and directly respond. We still
 * can send a command to the device the same way at any time. The message might get queued if it cannot be delivered
 * right away. But for this demo, we want to see some immediate results.
 */
@Startup
@ApplicationScoped
public class Processor {

    private static final Logger LOG = LoggerFactory.getLogger(Processor.class);

    private volatile String response = "pong";

    @Inject
    @Channel("response-changes")
    @Broadcast
    Emitter<String> responseChanges;

    public void changeResponse(String response) {
        this.response = response;
        this.responseChanges.send(this.response);
    }

    public String getResponse() {
        return this.response;
    }

    @Incoming("event-stream")
    @Outgoing("device-commands")
    @Broadcast
    public DeviceCommand process(DeviceEvent event) {

        var payload = event.getPayload();

        LOG.info("Received payload: {}", payload);

        if (!payload.startsWith("ping:")) {
            return null;
        }

        var responsePayload = response + payload.substring(payload.indexOf(':'));

        var command = new DeviceCommand();

        command.setDeviceId(event.getDeviceId());
        command.setPayload(responsePayload.getBytes(StandardCharsets.UTF_8));

        return command;

    }

}
