package io.drogue.iot.hackathon.endpoints;

import javax.inject.Inject;
import javax.ws.rs.BadRequestException;
import javax.ws.rs.Consumes;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.hackathon.Processor;
import io.drogue.iot.hackathon.data.DisplaySettings;
import io.drogue.iot.hackathon.registry.Registry;
import io.drogue.iot.hackathon.service.DeviceClaimService;
import io.quarkus.runtime.annotations.RegisterForReflection;
import io.quarkus.security.Authenticated;
import io.quarkus.security.identity.SecurityIdentity;

@Path("/api/commands/v1alpha1")
@Authenticated
public class CommandsResource {

    private static final Logger logger = LoggerFactory.getLogger(CommandsResource.class);

    @RegisterForReflection
    public static class DisplayState {
        public int brightness;

        public boolean enabled;
    }

    @Inject
    Processor processor;

    @Inject
    SecurityIdentity identity;

    @Inject
    DeviceClaimService service;

    @Inject
    Registry registry;

    @POST
    @Path("/display")
    @Consumes(MediaType.APPLICATION_JSON)
    @Produces()
    public void updateDisplay(DisplayState command) {

        var claim = this.service.getDeviceClaimFor(this.identity.getPrincipal().getName());

        logger.info("Request to send display command: {}", claim);

        if (claim.isEmpty()) {
            throw new BadRequestException("No claimed device");
        }

        var deviceName = claim.get().getId();
        var settings = new DisplaySettings();
        settings.device = deviceName;
        settings.enabled = command.enabled;
        settings.brightness = command.brightness;

        try {
           settings.address = Long.parseLong(claim.get().getProvisioningId());
        } catch (Exception e) {
           settings.address = 0L;
        }
        logger.info("Sending display command: {}", settings);
        this.processor.displayCommand(settings);
    }
}
