package io.drogue.iot.hackathon.model;

import static java.util.Optional.empty;
import static java.util.Optional.of;

import java.time.OffsetDateTime;
import java.util.Optional;

import com.google.common.base.MoreObjects;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class BasicFeature {
    private final OffsetDateTime lastUpdate;

    private final Object value;

    BasicFeature(final Object value, final OffsetDateTime lastUpdate) {
        this.value = value;
        this.lastUpdate = lastUpdate;
    }

    public Object getValue() {
        return this.value;
    }

    public OffsetDateTime getLastUpdate() {
        return this.lastUpdate;
    }

    @Override
    public String toString() {
        return MoreObjects.toStringHelper(this)
                .add("lastUpdate", this.lastUpdate)
                .add("value", this.value)
                .toString();
    }

    @SuppressWarnings("SameParameterValue")
    public <T> Optional<T> toTyped(final Class<T> clazz) {
        final var value = this.value;
        if (value == null) {
            return empty();
        }
        if (!clazz.isAssignableFrom(value.getClass())) {
            return empty();
        }

        return of(clazz.cast(value));
    }

    public Optional<Double> toDouble() {
        if (this.value instanceof Number) {
            return of(((Number) this.value).doubleValue());
        } else {
            return empty();
        }
    }

}
