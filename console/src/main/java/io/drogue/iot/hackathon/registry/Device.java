package io.drogue.iot.hackathon.registry;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonInclude;

@JsonIgnoreProperties(ignoreUnknown = true)
@JsonInclude(JsonInclude.Include.NON_NULL)
public class Device {
    private Metadata metadata;
    private DeviceSpec spec;

    private DeviceStatus status;

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

    public DeviceStatus getStatus() {
        return status;
    }

    public Device setStatus(DeviceStatus status) {
        this.status = status;
        return this;
    }
}
