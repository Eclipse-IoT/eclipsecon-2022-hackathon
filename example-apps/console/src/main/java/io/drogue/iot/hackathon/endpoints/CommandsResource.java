package io.drogue.iot.hackathon.endpoints;

import javax.inject.Inject;
import javax.ws.rs.BadRequestException;
import javax.ws.rs.Consumes;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import io.drogue.iot.hackathon.data.CommandPayload;
import io.drogue.iot.hackathon.data.LevelSet;
import io.drogue.iot.hackathon.data.OnOffSet;
import io.drogue.iot.hackathon.integration.DeviceCommand;
import io.drogue.iot.hackathon.service.DeviceClaim;
import io.smallrye.reactive.messaging.annotations.Broadcast;
import org.eclipse.microprofile.reactive.messaging.Channel;
import org.eclipse.microprofile.reactive.messaging.Emitter;
import org.eclipse.microprofile.reactive.messaging.OnOverflow;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.hackathon.registry.Registry;
import io.drogue.iot.hackathon.service.DeviceClaimService;
import io.quarkus.runtime.annotations.RegisterForReflection;
import io.quarkus.security.Authenticated;
import io.quarkus.security.identity.SecurityIdentity;

@Path("/api/commands/v1alpha1")
@Authenticated
public class CommandsResource {
    private static final Logger LOG = LoggerFactory.getLogger(CommandsResource.class);
    private static final Logger logger = LoggerFactory.getLogger(CommandsResource.class);

    @RegisterForReflection
    public static class DisplayState {
        public int brightness;
    }

    @RegisterForReflection
    public static class SpeakerState {
        public boolean enabled;
    }

    @Inject
    @Channel("device-commands")
    @Broadcast
    @OnOverflow(value = OnOverflow.Strategy.LATEST)
    Emitter<DeviceCommand> deviceCommands;

    @Inject
    SecurityIdentity identity;

    @Inject
    DeviceClaimService service;

    @Inject
    Registry registry;

    @POST
    @Path("/display")
    @Consumes(MediaType.APPLICATION_JSON)
    public void updateDisplay(DisplayState state) {
        var claim = this.service.getDeviceClaimFor(this.identity.getPrincipal().getName());
        logger.info("Request to send display command: {}", claim);
        if (claim.isEmpty()) {
            throw new BadRequestException("No claimed device");
        }

        var deviceName = claim.get().getId();
        var command = new CommandPayload(parseAddress(claim.get()));
        var level = new LevelSet();
        level.setLevel((short) state.brightness);
        command.setDisplay(level);
        sendCommand(deviceName, command);
    }

    @POST
    @Path("/speaker")
    @Consumes(MediaType.APPLICATION_JSON)
    @Produces()
    public void updateSpeaker(SpeakerState state) {
        var claim = this.service.getDeviceClaimFor(this.identity.getPrincipal().getName());
        logger.info("Request to send speaker command: {}", claim);
        if (claim.isEmpty()) {
            throw new BadRequestException("No claimed device");
        }

        var deviceName = claim.get().getId();
        var command = new CommandPayload(parseAddress(claim.get()));
        var onoff = new OnOffSet(state.enabled);
        command.setSpeaker(onoff);
        sendCommand(deviceName, command);
    }

    long parseAddress(DeviceClaim claim) {
        long address = 0L;
        try {
            address = Long.parseLong(claim.getProvisioningId(), 16);
        } catch (Exception e) {
            // Use default 0
        }
        return address;
    }


    void sendCommand(String deviceName, CommandPayload payload) {
        var command = new DeviceCommand();
        command.setDeviceId(deviceName);
        command.setPayload(payload);

        LOG.info("Sending command: {} to address {}", command, deviceName);

        this.deviceCommands.send(command);
    }


}
