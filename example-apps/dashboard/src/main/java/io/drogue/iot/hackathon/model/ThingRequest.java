package io.drogue.iot.hackathon.model;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class ThingRequest {
    public ThingRequestType type;

    public String thing;
}
