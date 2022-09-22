package io.drogue.iot.hackathon.data;

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
        return this.partial;
    }

    public DeviceState getState() {
        return this.state;
    }

    @Override
    public String toString() {
        return "DevicePayload{" +
                "partial=" + this.partial +
                ", state=" + this.state +
                '}';
    }

    public static DevicePayload empty() {
        return new DevicePayload(false, new DeviceState());
    }
}
