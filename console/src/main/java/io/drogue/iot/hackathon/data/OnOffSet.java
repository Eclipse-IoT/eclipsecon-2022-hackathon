package io.drogue.iot.hackathon.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class OnOffSet extends ModelElement {
    private final boolean on;

    public OnOffSet(boolean on) {
        this.on = on;
    }

    public boolean isOn() {
        return on;
    }

    @Override
    public String toString() {
        return "OnOffSet{" +
                "on=" + on +
                ", location=" + location +
                '}';
    }
}
