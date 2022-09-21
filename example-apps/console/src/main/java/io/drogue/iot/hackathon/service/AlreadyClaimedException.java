package io.drogue.iot.hackathon.service;

import javax.ws.rs.WebApplicationException;
import javax.ws.rs.core.Response;

public class AlreadyClaimedException extends WebApplicationException {

    public AlreadyClaimedException(final String deviceId) {
        super(String.format("Device %s is already claimed", deviceId), Response.Status.CONFLICT);
    }
}
