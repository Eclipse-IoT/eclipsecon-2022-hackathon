package io.drogue.iot.hackathon.model;

import static java.util.Optional.empty;
import static java.util.Optional.of;

import java.time.OffsetDateTime;
import java.util.Optional;

import com.google.common.base.MoreObjects;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class BasicFeature {
    public OffsetDateTime lastChange;

    public Object value;

    @Override
    public String toString() {
        return MoreObjects.toStringHelper(this)
                .add("lastChange", this.lastChange)
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
