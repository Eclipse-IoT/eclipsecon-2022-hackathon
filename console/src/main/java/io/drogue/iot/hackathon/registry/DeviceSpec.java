package io.drogue.iot.hackathon.registry;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

import java.util.List;

@JsonIgnoreProperties(ignoreUnknown = true)
public class DeviceSpec {
    private GatewaySelector gatewaySelector;
    private List<String> alias;

    private BtMeshSpec btmesh;

    public GatewaySelector getGatewaySelector() {
        return gatewaySelector;
    }

    public void setGatewaySelector(GatewaySelector gatewaySelector) {
        this.gatewaySelector = gatewaySelector;
    }

    public List<String> getAlias() {
        return alias;
    }

    public void setAlias(List<String> alias) {
        this.alias = alias;
    }

    public BtMeshSpec getBtmesh() {
        return btmesh;
    }

    public void setBtmesh(BtMeshSpec btmesh) {
        this.btmesh = btmesh;
    }
}
