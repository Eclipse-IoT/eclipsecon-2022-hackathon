package io.drogue.iot.hackathon.registry;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

@JsonIgnoreProperties(ignoreUnknown = true)
public class BtMeshStatus {
    private Long address;

    public Long getAddress() {
        return address;
    }

    public BtMeshStatus setAddress(Long address) {
        this.address = address;
        return this;
    }
}
