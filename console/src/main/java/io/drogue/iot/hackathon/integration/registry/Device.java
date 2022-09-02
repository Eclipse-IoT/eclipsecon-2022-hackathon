package io.drogue.iot.hackathon.integration.registry;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

@JsonIgnoreProperties(ignoreUnknown = true)
public class Device {
    private Metadata metadata;
    private DeviceSpec spec;

    public Metadata getMetadata() {
        return metadata;
    }

    public Device setMetadata(Metadata metadata) {
        this.metadata = metadata;
        return this;
    }

    public DeviceSpec getSpec() {
        return spec;
    }

    public Device setSpec(DeviceSpec spec) {
        this.spec = spec;
        return this;
    }
}
