package io.drogue.iot.hackathon.service;

import javax.ws.rs.WebApplicationException;
import javax.ws.rs.core.Response;

public class InvalidClaimIdException extends WebApplicationException {
    public InvalidClaimIdException(final String claimId) {
        super(String.format("Invalid Claim Id %s ", claimId), Response.Status.NOT_FOUND);
    }
}
