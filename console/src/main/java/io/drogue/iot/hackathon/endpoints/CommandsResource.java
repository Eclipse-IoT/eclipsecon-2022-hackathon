package io.drogue.iot.hackathon.endpoints;

import javax.inject.Inject;
import javax.ws.rs.BadRequestException;
import javax.ws.rs.Consumes;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import io.drogue.iot.hackathon.Processor;
import io.drogue.iot.hackathon.registry.Registry;
import io.drogue.iot.hackathon.service.DeviceClaimService;
import io.drogue.iot.hackathon.ui.DisplaySettings;
import io.quarkus.security.Authenticated;
import io.quarkus.security.identity.SecurityIdentity;

@Path("/api/commands/v1alpha1")
@Authenticated
public class CommandsResource {

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

        var claim = service.getDeviceClaimFor(identity.getPrincipal().getName());

        if (claim.isEmpty()) {
            throw new BadRequestException("No claimed device");
        }

        var device = registry.getDevice(claim.get().id);
        if (device.getStatus() != null) {
            if (device.getStatus().getBtmesh() != null) {
                if (device.getStatus().getBtmesh().getAddress() != null) {
                    String address = String.format("%04x", device.getStatus().getBtmesh().getAddress());

                    var settings = new DisplaySettings();
                    settings.device = address;
                    settings.enabled = command.enabled;
                    settings.brightness = command.brightness;

                    this.processor.updateDisplaySettings(settings);
                }
            }
        }
    }
}
