package io.drogue.iot.hackathon;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class ProvisioningCommand {
    private String device;

    public ProvisioningCommand(String device) {
        this.device = device;
    }

    public String getDevice() {
        return device;
    }
}
