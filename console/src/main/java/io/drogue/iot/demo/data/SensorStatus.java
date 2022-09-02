package io.drogue.iot.demo.data;

import io.quarkus.runtime.annotations.RegisterForReflection;
import io.vertx.codegen.Model;

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
