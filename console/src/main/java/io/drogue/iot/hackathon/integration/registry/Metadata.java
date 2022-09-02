package io.drogue.iot.hackathon.integration.registry;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

@JsonIgnoreProperties(ignoreUnknown = true)
public class Metadata {
    private String name;

    public String getName() {
        return name;
    }

    public Metadata setName(String name) {
        this.name = name;
        return this;
    }
}
