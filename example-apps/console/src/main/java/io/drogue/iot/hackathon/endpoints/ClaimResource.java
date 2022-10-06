package io.drogue.iot.hackathon.endpoints;

import java.util.Optional;
import java.util.UUID;

import javax.inject.Inject;
import javax.transaction.Transactional;
import javax.ws.rs.Consumes;
import javax.ws.rs.DELETE;
import javax.ws.rs.GET;
import javax.ws.rs.PUT;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.QueryParam;
import javax.ws.rs.WebApplicationException;
import javax.ws.rs.core.MediaType;

import io.drogue.iot.hackathon.events.EventDispatcher;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

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
    SecurityIdentity identity;

    @Inject
    Registry registry;

    @Inject
    EventDispatcher dispatcher;

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
    public void claimDevice(@QueryParam("claimId") final String claimId) {
        var canCreate = this.identity.hasRole("device-admin");
        this.claimDevice(claimId, this.identity.getPrincipal().getName(), canCreate);
    }

    @PUT
    @Path("/simulator")
    public void claimSimulator() {
        logger.warn("Failed");
        try {
            this.claimSimulatorDevice(this.identity.getPrincipal().getName());
        } catch (Exception e) {
            logger.warn("Failed to claim simulator", e);
            throw e;
        }
    }

    @DELETE
    @Produces(MediaType.APPLICATION_JSON)
    public Boolean releaseDevice() {
        releaseDevice(this.identity.getPrincipal().getName());
        var result = this.service.releaseDevice(this.identity.getPrincipal().getName());
        logger.info("Released device: {}", result);
        return result;
    }


    @Transactional
    void claimDevice(final String claimId, final String userId, final boolean canCreate) {
        this.service.claimDevice(claimId, userId, canCreate);
    }

    @Transactional
    void claimSimulatorDevice(final String userId) {
        var id = "simulator-" + UUID.randomUUID();
        var pwd = UUID.randomUUID().toString();
        this.service.claimDevice(id, userId, true);
        this.registry.createSimulatorDevice(id, pwd);
    }

    @Transactional
    public void releaseDevice(final String userId) {
        var claim = this.service.getDeviceClaimFor(userId);
        claim.ifPresent(deviceClaim -> {
            if (deviceClaim.getId().startsWith("simulator-")) {
                try {
                    this.registry.deleteDevice(deviceClaim.getId());
                } catch (WebApplicationException e) {
                    if (e.getResponse().getStatus() != 404) {
                        // ignore 404
                        throw e;
                    }
                }
            }
            this.dispatcher.releaseDevice(deviceClaim.getId());
        });

        this.service.releaseDevice(userId);
    }
}
