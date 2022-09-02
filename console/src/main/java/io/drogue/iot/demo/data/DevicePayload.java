package io.drogue.iot.demo.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class DevicePayload {
    private final boolean partial;
    private final DeviceState state;

    public DevicePayload(boolean partial, DeviceState state) {
        this.partial = partial;
        this.state = state;
    }

    public boolean isPartial() {
        return partial;
    }

    public DeviceState getState() {
        return state;
    }

    @Override
    public String toString() {
        return "DevicePayload{" +
                "partial=" + partial +
                ", state=" + state +
                '}';
    }
}
