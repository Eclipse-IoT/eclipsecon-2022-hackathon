package io.drogue.iot.hackathon.model;

import java.time.OffsetDateTime;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class ReportedFeature extends BasicFeature {
    ReportedFeature(final Object value, final OffsetDateTime lastUpdate) {
        super(value, lastUpdate);
    }
}
