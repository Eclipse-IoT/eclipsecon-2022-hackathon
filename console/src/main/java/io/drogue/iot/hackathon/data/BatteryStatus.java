package io.drogue.iot.hackathon.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class BatteryStatus extends ModelElement {
    private byte level;

    private BatteryFlags flags;

    public byte getLevel() {
        return this.level;
    }

    public void setLevel(byte level) {
        this.level = level;
    }

    public BatteryFlags getFlags() {
        return this.flags;
    }

    public void setFlags(BatteryFlags flags) {
        this.flags = flags;
    }

    @Override
    public String toString() {
        return "BatteryStatus{" +
                "level=" + this.level +
                ", flags=" + this.flags +
                ", location=" + this.location +
                '}';
    }
}
