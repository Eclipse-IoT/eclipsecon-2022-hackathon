package io.drogue.iot.hackathon.model;

import java.util.HashMap;
import java.util.Map;

import com.google.common.base.MoreObjects;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class Thing {
    public Metadata metadata;

    public Map<String, ReportedFeature> reportedState = new HashMap<>();

    public Map<String, SyntheticFeature> syntheticState = new HashMap<>();

    @Override
    public String toString() {
        return MoreObjects.toStringHelper(this)
                .add("metadata", this.metadata)
                .add("reportedState", this.reportedState)
                .toString();
    }
}
