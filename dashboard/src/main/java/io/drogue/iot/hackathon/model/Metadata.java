package io.drogue.iot.hackathon.model;

import com.google.common.base.MoreObjects;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class Metadata {
    public String name;

    public String application;

    @Override
    public String toString() {
        return MoreObjects.toStringHelper(this)
                .add("name", this.name)
                .add("application", this.application)
                .toString();
    }
}
