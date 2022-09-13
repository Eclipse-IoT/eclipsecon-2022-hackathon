package io.drogue.iot.hackathon.registry;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

import java.util.List;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
@JsonIgnoreProperties(ignoreUnknown = true)
public class GatewaySelector {
    private List<String> matchNames;

    public List<String> getMatchNames() {
        return matchNames;
    }

    public GatewaySelector setMatchNames(List<String> matchNames) {
        this.matchNames = matchNames;
        return this;
    }
}
