package io.drogue.iot.hackathon.data;

import java.time.Instant;

import io.quarkus.runtime.annotations.RegisterForReflection;

/**
 * An incoming device message.
 */
@RegisterForReflection
public class DeviceEvent {
    private String deviceId;

    private Instant timestamp;
    private DevicePayload payload;

    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public String getDeviceId() {
        return this.deviceId;
    }

    public void setPayload(DevicePayload payload) {
        this.payload = payload;
    }

    public DevicePayload getPayload() {
        return this.payload;
    }

    public Instant getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(Instant timestamp) {
        this.timestamp = timestamp;
    }

    @Override
    public String toString() {
        return "DeviceEvent{" +
                "deviceId='" + deviceId + '\'' +
                ", timestamp=" + timestamp +
                ", payload=" + payload +
                '}';
    }
}
