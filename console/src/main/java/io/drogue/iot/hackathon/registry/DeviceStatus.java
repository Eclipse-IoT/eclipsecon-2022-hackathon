package io.drogue.iot.hackathon.registry;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

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
