package io.drogue.iot.demo;

import java.util.ArrayDeque;
import java.util.Queue;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;

import io.drogue.iot.demo.data.OnOffSet;
import io.drogue.iot.demo.ui.DisplaySettings;
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

    private final Queue<DisplaySettings> settings = new ArrayDeque<>(10);
    private volatile DisplaySettings displaySettings = null;

    @Inject
    @Channel("display-changes")
    @Broadcast
    Emitter<DisplaySettings> displayChanges;

    public void updateDisplaySettings(DisplaySettings settings) {
        LOG.info("Changing settings to {}", settings);
        this.displaySettings = settings;
        this.displayChanges.send(this.displaySettings);
        this.settings.add(settings);
    }

    public DisplaySettings getDisplaySettings() {
        return this.displaySettings;
    }


    @Incoming("event-stream")
    @Outgoing("device-commands")
    @Broadcast
    public DeviceCommand process(DeviceEvent event) {

        var payload = event.getPayload();

        var response = this.settings.poll();
        if (response == null) {
            return null;
        }

        var display = new OnOffSet(response.enabled);
        display.setLocation((short)0x100);
        var commandPayload = new CommandPayload(display);
        var command = new DeviceCommand();
        command.setDeviceId(event.getDeviceId());
        command.setPayload(commandPayload);

        LOG.info("Sending command: {}", command);

        return command;
    }

}
