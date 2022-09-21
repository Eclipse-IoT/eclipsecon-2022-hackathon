package io.drogue.iot.hackathon.service;

import javax.ws.rs.WebApplicationException;
import javax.ws.rs.core.Response;

public class StillClaimedException extends WebApplicationException {

    public StillClaimedException(String claimId) {
        super(String.format("Claim %s is still in use", claimId), Response.Status.CONFLICT);
    }

}
