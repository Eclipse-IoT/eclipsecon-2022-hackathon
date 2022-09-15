package io.drogue.iot.hackathon.registry;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

import java.util.List;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
@JsonIgnoreProperties(ignoreUnknown = true)
public class DeviceAliases {
    private List<String> aliases;

    public List<String> getAliases() {
        return aliases;
    }

    public DeviceAliases setAliases(List<String> aliases) {
        this.aliases = aliases;
        return this;
    }
}
