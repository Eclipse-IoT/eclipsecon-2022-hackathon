package io.drogue.iot.hackathon.endpoints;

import java.util.Optional;

import javax.inject.Inject;
import javax.ws.rs.Consumes;
import javax.ws.rs.DELETE;
import javax.ws.rs.GET;
import javax.ws.rs.PUT;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.QueryParam;
import javax.ws.rs.core.MediaType;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.hackathon.Processor;
import io.drogue.iot.hackathon.registry.Password;
import io.drogue.iot.hackathon.registry.Registry;
import io.drogue.iot.hackathon.service.DeviceClaim;
import io.drogue.iot.hackathon.service.DeviceClaimService;
import io.quarkus.security.Authenticated;
import io.quarkus.security.identity.SecurityIdentity;

@Path("/api/deviceClaims/v1alpha1")
@Consumes()
@Produces()
@Authenticated
public class ClaimResource {

    private static final Logger logger = LoggerFactory.getLogger(ClaimResource.class);

    @Inject
    DeviceClaimService service;

    @Inject
    Processor processor;

    @Inject
    SecurityIdentity identity;

    @Inject
    Registry registry;

    @GET
    @Produces(MediaType.APPLICATION_JSON)
    public Optional<DeviceClaim> getDeviceClaim() {
        var claim = this.service.getDeviceClaimFor(this.identity.getPrincipal().getName());

        if (claim.isPresent()) {
            if (claim.get().getId().startsWith("simulator-")) {
                // if this is a simulator, try to find a password
                var device = this.registry.getDevice(claim.get().getId()).orElse(null);
                if (device != null && device.getSpec() != null && device.getSpec().getCredentials() != null && device.getSpec().getCredentials().getCredentials() != null) {
                    var pwd = device.getSpec().getCredentials().getCredentials()
                            .stream()
                            .filter(Password.class::isInstance)
                            .map(Password.class::cast)
                            .filter(p -> p.getAlgorithm() == null || p.getAlgorithm().equals("plain"))
                            .map(Password::getValue)
                            .findFirst();
                    claim = Optional.of(claim.get().withPassword(pwd));
                }
            }
        }

        return claim;
    }

    @PUT
    public void claimDevice(@QueryParam("deviceId") final String deviceId) {
        var canCreate = this.identity.hasRole("device-admin");
        this.processor.claimDevice(deviceId, this.identity.getPrincipal().getName(), canCreate);
    }

    @PUT
    @Path("/simulator")
    public void claimSimulator() {
        logger.warn("Failed");
        try {
            this.processor.claimSimulatorDevice(this.identity.getPrincipal().getName());
        } catch (Exception e) {
            logger.warn("Failed to claim simulator", e);
            throw e;
        }
    }

    @DELETE
    @Produces(MediaType.APPLICATION_JSON)
    public Boolean releaseDevice() {
        this.processor.releaseDevice(this.identity.getPrincipal().getName());
        var result = this.service.releaseDevice(this.identity.getPrincipal().getName());
        logger.info("Released device: {}", result);
        return result;
    }
}
