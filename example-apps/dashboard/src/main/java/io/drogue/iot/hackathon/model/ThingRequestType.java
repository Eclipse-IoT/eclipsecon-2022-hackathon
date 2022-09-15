package io.drogue.iot.hackathon.model;

import com.fasterxml.jackson.annotation.JsonProperty;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public enum ThingRequestType {
    @JsonProperty("subscribe")
    Subscribe,
    @JsonProperty("unsubscribe")
    Unsubscribe
}
