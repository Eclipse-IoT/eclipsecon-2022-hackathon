package io.drogue.iot.hackathon;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.transaction.Transactional;

import org.eclipse.microprofile.reactive.messaging.Channel;
import org.eclipse.microprofile.reactive.messaging.Emitter;
import org.eclipse.microprofile.reactive.messaging.Incoming;
import org.eclipse.microprofile.reactive.messaging.OnOverflow;
import org.eclipse.microprofile.reactive.messaging.Outgoing;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.hackathon.commands.DeviceCommand;
import io.drogue.iot.hackathon.data.CommandPayload;
import io.drogue.iot.hackathon.data.DeviceEvent;
import io.drogue.iot.hackathon.data.OnOffSet;
import io.drogue.iot.hackathon.registry.Registry;
import io.drogue.iot.hackathon.service.DeviceClaimService;
import io.drogue.iot.hackathon.ui.DisplaySettings;
import io.quarkus.runtime.Startup;
import io.smallrye.reactive.messaging.annotations.Broadcast;

import java.util.Collections;

/**
 * Process device events.
 * <p>
 * This is main logic in this application. Processing happens in the {@link #process(DeviceEvent)} method.
 * <p>
 * It receives messages from the Drogue IoT MQTT integration, pre-processed by the {@link
 * io.drogue.iot.hackathon.integration.Receiver}. It can return {@code null} to do nothing, or a {@link CommandPayload} to
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

    @Inject
    @Channel("display-changes")
    @Broadcast
    Emitter<DisplaySettings> displayChanges;

    public void updateDisplaySettings(DisplaySettings settings) {
        this.displayChanges.send(settings);
    }

    @Incoming("display-changes")
    @Outgoing("device-commands")
    @Broadcast
    @OnOverflow(value = OnOverflow.Strategy.LATEST)
    public DeviceCommand displayCommand(DisplaySettings settings) {
        var display = new OnOffSet(settings.enabled);
        display.setLocation((short) 0x100);
        var commandPayload = new CommandPayload(display);
        var command = new DeviceCommand();
        command.setDeviceId(settings.device);
        command.setPayload(commandPayload);

        LOG.info("Sending command: {}", command);

        return command;
    }

    @Incoming("event-stream")
    public void process(DeviceEvent event) {
        var payload = event.getPayload();

        LOG.info("Received sensor data: {}", payload);
    }

    @Inject
    Registry registry;

    @Inject
    DeviceClaimService service;

    @Transactional
    public void claimDevice(final String claimId, final String userId, final boolean canCreate) {
        var claim = service.claimDevice(claimId, userId, canCreate);
        registry.createDevice(claimId, claim.deviceId);
    }

    @Transactional
    public void releaseDevice(final String userId) {
        var claim = service.getDeviceClaimFor(userId);
        claim.ifPresent(deviceClaim -> registry.deleteDevice(deviceClaim.id));
        service.releaseDevice(userId);
    }
}
