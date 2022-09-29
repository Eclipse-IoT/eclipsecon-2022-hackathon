package io.drogue.iot.hackathon.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class LevelSet extends ModelElement {
    private Short level;

    public Short getLevel() {
        return level;
    }

    public void setLevel(Short level) {
        this.level = level;
    }

    @Override
    public String toString() {
        return "LevelSet{" +
                "level=" + level +
                '}';
    }
}
