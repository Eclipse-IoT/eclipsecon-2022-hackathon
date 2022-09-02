package io.drogue.iot.hackathon.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class ModelElement {
    protected short location;

    public short getLocation() {
        return location;
    }

    public void setLocation(short location) {
        this.location = location;
    }

    @Override
    public String toString() {
        return "ModelElement{" +
                "location=" + location +
                '}';
    }
}
