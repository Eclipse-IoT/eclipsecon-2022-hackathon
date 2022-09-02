package io.drogue.iot.hackathon.ui;

import io.drogue.iot.hackathon.Processor;
import io.drogue.iot.hackathon.integration.registry.Registry;
import io.drogue.iot.hackathon.ui.ClaimRequest;

import javax.inject.Inject;
import javax.ws.rs.Consumes;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

@Path("/claim")
public class ClaimResource {
    @Inject
    Processor processor;

    @POST
    @Consumes(MediaType.APPLICATION_JSON)
    public void claimDevice(ClaimRequest request) {
        this.processor.claimDevice(request.claim_id);
    }
}
