package io.drogue.iot.hackathon.registry;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
@JsonIgnoreProperties(ignoreUnknown = true)
public class DeviceStatus {
    private BtMeshStatus btmesh;

    public BtMeshStatus getBtmesh() {
        return btmesh;
    }

    public DeviceStatus setBtmesh(BtMeshStatus btmesh) {
        this.btmesh = btmesh;
        return this;
    }
}
