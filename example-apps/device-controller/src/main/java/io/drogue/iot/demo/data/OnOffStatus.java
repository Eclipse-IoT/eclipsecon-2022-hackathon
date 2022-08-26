package io.drogue.iot.demo.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class OnOffStatus extends ModelElement {
    private boolean on;

    public boolean isOn() {
        return on;
    }

    public void setOn(boolean on) {
        this.on = on;
    }

    @Override
    public String toString() {
        return "OnOffStatus{" +
                "on=" + on +
                ", location=" + location +
                '}';
    }
}
