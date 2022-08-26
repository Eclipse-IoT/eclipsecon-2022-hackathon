package io.drogue.iot.demo;

import java.nio.charset.StandardCharsets;
import java.util.ArrayDeque;
import java.util.Queue;
import java.util.concurrent.BlockingQueue;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;

import io.drogue.iot.demo.data.OnOffSet;
import org.eclipse.microprofile.reactive.messaging.Channel;
import org.eclipse.microprofile.reactive.messaging.Emitter;
import org.eclipse.microprofile.reactive.messaging.Incoming;
import org.eclipse.microprofile.reactive.messaging.Outgoing;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.demo.data.CommandPayload;
import io.drogue.iot.demo.data.DeviceEvent;
import io.quarkus.runtime.Startup;
import io.smallrye.reactive.messaging.annotations.Broadcast;

/**
 * Process device events.
 * <p>
 * This is main logic in this application. Processing happens in the {@link #process(DeviceEvent)} method.
 * <p>
 * It receives messages from the Drogue IoT MQTT integration, pre-processed by the {@link
 * io.drogue.iot.demo.integration.Receiver}. It can return {@code null} to do nothing, or a {@link CommandPayload} to
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

    private final Queue<Boolean> toggles = new ArrayDeque<>(10);
    private volatile Boolean response = false;

    @Inject
    @Channel("response-changes")
    @Broadcast
    Emitter<Boolean> responseChanges;

    public void toggleDisplay() {
        LOG.info("Changing response to {}", response);
        this.response = !response;
        this.responseChanges.send(this.response);
        this.toggles.add(this.response);
    }

    public Boolean getResponse() {
        return this.response;
    }


    @Incoming("event-stream")
    @Outgoing("device-commands")
    @Broadcast
    public DeviceCommand process(DeviceEvent event) {

        var payload = event.getPayload();

        LOG.info("Received payload: {}", payload);

        var response = this.toggles.poll();
        if (response == null) {
            return null;
        }

        var display = new OnOffSet(response);
        var commandPayload = new CommandPayload(display);
        var command = new DeviceCommand();
        command.setDeviceId(event.getDeviceId());
        command.setPayload(commandPayload);

        LOG.info("Sending command: {}", command);

        return command;
    }

}
