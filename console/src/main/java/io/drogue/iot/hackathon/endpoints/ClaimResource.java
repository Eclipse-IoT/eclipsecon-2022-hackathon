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
import io.drogue.iot.hackathon.service.DeviceClaim;
import io.drogue.iot.hackathon.service.DeviceClaimService;
import io.quarkus.security.Authenticated;
import io.quarkus.security.identity.SecurityIdentity;

@Path("/api/deviceClaims/v1alpha1")
@Consumes(MediaType.APPLICATION_JSON)
@Produces(MediaType.APPLICATION_JSON)
@Authenticated
public class ClaimResource {

    private static final Logger logger = LoggerFactory.getLogger(ClaimResource.class);

    @Inject
    DeviceClaimService service;

    @Inject
    Processor processor;

    @Inject
    SecurityIdentity identity;

    @GET
    public Optional<DeviceClaim> getDeviceClaim() {
        return this.service.getDeviceClaimFor(this.identity.getPrincipal().getName());
    }

    @PUT
    public void claimDevice(@QueryParam("deviceId") final String deviceId) {
        var canCreate = this.identity.hasRole("device-admin");
        this.processor.claimDevice(deviceId, this.identity.getPrincipal().getName(), canCreate);
    }

    @DELETE
    public Boolean releaseDevice() {
        this.processor.releaseDevice(this.identity.getPrincipal().getName());
        var result = this.service.releaseDevice(this.identity.getPrincipal().getName());
        logger.info("Released device: {}", result);
        return result;
    }
}
