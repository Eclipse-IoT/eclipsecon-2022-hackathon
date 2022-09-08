package io.drogue.iot.hackathon.commands;

import io.quarkus.runtime.annotations.RegisterForReflection;

import java.util.List;

@RegisterForReflection
public class ProvisioningCommand {
    private final ProvisioningOperation operation;
    private final String device;
    private final List<String> aliases;

    public ProvisioningCommand(ProvisioningOperation operation, String device, List<String> aliases) {
        this.operation = operation;
        this.device = device;
        this.aliases = aliases;
    }

    public ProvisioningOperation getOperation() {
        return operation;
    }

    public String getDevice() {
        return device;
    }

    public List<String> getAliases() {
        return aliases;
    }
}
