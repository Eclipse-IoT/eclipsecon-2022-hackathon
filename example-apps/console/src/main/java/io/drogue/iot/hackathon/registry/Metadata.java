package io.drogue.iot.hackathon.registry;

import java.util.Map;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonInclude;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
@JsonIgnoreProperties(ignoreUnknown = true)
public class Metadata {
    private String name;

    private String application;

    @JsonInclude(JsonInclude.Include.NON_EMPTY)
    private Map<String, String> labels;

    public String getName() {
        return this.name;
    }

    public Metadata setName(String name) {
        this.name = name;
        return this;
    }

    public String getApplication() {
        return this.application;
    }

    public void setApplication(String application) {
        this.application = application;
    }

    public Map<String, String> getLabels() {
        return this.labels;
    }

    public void setLabels(Map<String, String> labels) {
        this.labels = labels;
    }
}
