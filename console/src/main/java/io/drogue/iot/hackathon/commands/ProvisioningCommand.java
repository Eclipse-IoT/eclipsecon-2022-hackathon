package io.drogue.iot.hackathon.commands;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class ProvisioningCommand {
    private final String device;
    private final String address;

    public ProvisioningCommand(String device, String address) {
        this.device = device;
        this.address = address;
    }

    public String getDevice() {
        return device;
    }

    public String getAddress() {
        return address;
    }
}
