package io.drogue.iot.hackathon.service;

import java.util.Objects;
import java.util.Optional;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.google.common.base.MoreObjects;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
@JsonInclude(JsonInclude.Include.NON_EMPTY)
public class DeviceClaim {
    public final String id;

    public final String deviceId;

    public final Optional<String> password;

    public DeviceClaim(String id, String deviceId) {
        this.id = id;
        this.deviceId = deviceId;
        this.password = Optional.empty();
    }

    public DeviceClaim(String id, String deviceId, Optional<String> password) {
        this.id = id;
        this.deviceId = deviceId;
        this.password = Objects.requireNonNull(password);
    }

    public String getId() {
        return this.id;
    }

    public String getDeviceId() {
        return this.deviceId;
    }

    public Optional<String> getPassword() {
        return this.password;
    }

    @Override
    public String toString() {
        return MoreObjects.toStringHelper(this)
                .add("id", this.id)
                .add("deviceId", this.deviceId)
                .add("password", this.password.isPresent() ? "***" : "<empty>")
                .toString();
    }

    public DeviceClaim withPassword(final Optional<String> password) {
        return new DeviceClaim(this.id, this.deviceId, password);
    }
}
