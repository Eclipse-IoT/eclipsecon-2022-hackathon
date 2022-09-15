package io.drogue.iot.hackathon.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class SensorStatus extends ModelElement {
    private SensorPayload payload;

    public SensorPayload getPayload() {
        return payload;
    }

    public void setPayload(SensorPayload payload) {
        this.payload = payload;
    }

    @Override
    public String toString() {
        return "SensorStatus{" +
                "payload=" + payload +
                ", location=" + location +
                '}';
    }
}
