package io.drogue.iot.hackathon.events;

import java.time.Instant;

import io.drogue.iot.hackathon.data.DeviceState;
import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class State {
    private final Instant lastUpdate;

    private final DeviceState deviceState;

    public State(Instant lastUpdate, DeviceState deviceState) {
        this.lastUpdate = lastUpdate;
        this.deviceState = deviceState;
    }

    public DeviceState getDeviceState() {
        return this.deviceState;
    }

    public Instant getLastUpdate() {
        return this.lastUpdate;
    }

    public State merge(State other) {
        var time = this.lastUpdate;
        if (time.isBefore(other.lastUpdate)) {
            time = other.lastUpdate;
        }
        return new State(time, this.deviceState.merge(other.deviceState));
    }
}
