package io.drogue.iot.hackathon.registry;

import java.util.List;

import com.fasterxml.jackson.annotation.JsonInclude;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class CredentialsSpec {
    @JsonInclude(JsonInclude.Include.NON_EMPTY)
    private List<Credentials> credentials;

    public List<Credentials> getCredentials() {
        return this.credentials;
    }

    public void setCredentials(List<Credentials> credentials) {
        this.credentials = credentials;
    }

}
