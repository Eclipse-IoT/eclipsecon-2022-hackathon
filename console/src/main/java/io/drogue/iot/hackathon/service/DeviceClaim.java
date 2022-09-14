package io.drogue.iot.hackathon.service;

import java.util.Objects;
import java.util.Optional;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.google.common.base.MoreObjects;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
@JsonInclude(JsonInclude.Include.NON_EMPTY)
public class DeviceClaim {
    private final String id;

    private final String provisioningId;

    private final Optional<String> password;

    public DeviceClaim(String id, String provisioningId) {
        this.id = id;
        this.provisioningId = provisioningId;
        this.password = Optional.empty();
    }

    public DeviceClaim(String id, String provisioningId, Optional<String> password) {
        this.id = id;
        this.provisioningId = provisioningId;
        this.password = Objects.requireNonNull(password);
    }

    public String getId() {
        return this.id;
    }

    public String getProvisioningId() {
        return this.provisioningId;
    }

    public Optional<String> getPassword() {
        return this.password;
    }

    @Override
    public String toString() {
        return MoreObjects.toStringHelper(this)
                .add("id", this.id)
                .add("provisioningId", this.provisioningId)
                .add("password", this.password.isPresent() ? "***" : "<empty>")
                .toString();
    }

    /**
     * Create a new device claim, overriding the password information.
     *
     * @param password The password to set.
     * @return The new instance.
     */
    public DeviceClaim withPassword(final Optional<String> password) {
        return new DeviceClaim(this.id, this.provisioningId, password);
    }
}
