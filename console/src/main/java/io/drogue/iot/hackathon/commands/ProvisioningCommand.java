package io.drogue.iot.hackathon.commands;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class ProvisioningCommand {
    private final String id;
    private final String device;
    public ProvisioningCommand(String id, String device) {
        this.id = id;
        this.device = device;
    }

    public String getDevice() {
        return device;
    }

    public String getId() {
        return id;
    }
}
