package io.drogue.iot.hackathon.model;

import java.time.OffsetDateTime;

import com.google.common.base.MoreObjects;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class BasicFeature {
    public OffsetDateTime lastChange;

    public Object value;

    @Override
    public String toString() {
        return MoreObjects.toStringHelper(this)
                .add("lastChange", this.lastChange)
                .add("value", this.value)
                .toString();
    }
}
