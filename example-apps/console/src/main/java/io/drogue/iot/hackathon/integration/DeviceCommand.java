package io.drogue.iot.hackathon.integration;

import com.fasterxml.jackson.databind.ObjectMapper;

import io.drogue.iot.hackathon.data.CommandPayload;
import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class DeviceCommand {
    private String deviceId;

    private CommandPayload payload;

    public String getDeviceId() {
        return this.deviceId;
    }

    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public byte[] getPayload() {
        var m = new ObjectMapper();
        try {
            return m.writeValueAsBytes(this.payload);
        } catch (Exception e) {
            e.printStackTrace();
            return null;
        }
    }

    public void setPayload(CommandPayload payload) {
        this.payload = payload;
    }

    @Override
    public String toString() {
        return "DeviceCommand{" +
                "deviceId='" + this.deviceId + '\'' +
                ", payload=" + this.payload +
                '}';
    }
}
