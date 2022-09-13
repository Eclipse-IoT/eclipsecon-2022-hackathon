package io.drogue.iot.hackathon.registry;

import java.util.List;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonInclude;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
@JsonIgnoreProperties(ignoreUnknown = true)
@JsonInclude(JsonInclude.Include.NON_NULL)
public class DeviceSpec {
    private GatewaySelector gatewaySelector;

    private List<String> alias;

    private BtMeshSpec btmesh;

    private CredentialsSpec credentials;

    public GatewaySelector getGatewaySelector() {
        return this.gatewaySelector;
    }

    public void setGatewaySelector(GatewaySelector gatewaySelector) {
        this.gatewaySelector = gatewaySelector;
    }

    public List<String> getAlias() {
        return this.alias;
    }

    public void setAlias(List<String> alias) {
        this.alias = alias;
    }

    public BtMeshSpec getBtmesh() {
        return this.btmesh;
    }

    public void setBtmesh(BtMeshSpec btmesh) {
        this.btmesh = btmesh;
    }

    public CredentialsSpec getCredentials() {
        return this.credentials;
    }

    public void setCredentials(CredentialsSpec credentials) {
        this.credentials = credentials;
    }
}
