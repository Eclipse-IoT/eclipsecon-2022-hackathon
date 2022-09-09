package io.drogue.iot.hackathon.registry;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

@JsonIgnoreProperties(ignoreUnknown = true)
public class DeviceSpec {
    private GatewaySelector gatewaySelector;
    private DeviceAliases alias;

    public GatewaySelector getSelector() {
        return selector;
    }

    public DeviceSpec setSelector(GatewaySelector selector) {
        this.selector = selector;
        return this;
    }

    public DeviceAliases getAlias() {
        return alias;
    }

    public DeviceSpec setAlias(DeviceAliases alias) {
        this.alias = alias;
        return this;
    }
}
