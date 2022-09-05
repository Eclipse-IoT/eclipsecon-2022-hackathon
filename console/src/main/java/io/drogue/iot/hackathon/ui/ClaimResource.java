package io.drogue.iot.hackathon.ui;

import io.drogue.iot.hackathon.Processor;

import javax.inject.Inject;
import javax.ws.rs.Consumes;
import javax.ws.rs.FormParam;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.Context;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Response;
import javax.ws.rs.core.UriInfo;

@Path("/claim")
public class ClaimResource {
    @Inject
    Processor processor;

    @Context
    UriInfo uri;

    @POST
    @Consumes(MediaType.APPLICATION_FORM_URLENCODED)
    @Produces(MediaType.TEXT_HTML)
    public Response claimDevice(@FormParam("claim_id") String claim_id) {
        this.processor.claimDevice(claim_id);
        return Response.seeOther(uri.getBaseUri()).build();
    }
}
