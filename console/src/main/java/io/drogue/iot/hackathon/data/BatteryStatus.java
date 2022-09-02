package io.drogue.iot.hackathon.data;

public class BatteryStatus extends ModelElement {
    private byte level;
    private BatteryFlags flags;

    public byte getLevel() {
        return level;
    }

    public void setLevel(byte level) {
        this.level = level;
    }

    public BatteryFlags getFlags() {
        return flags;
    }

    public void setFlags(BatteryFlags flags) {
        this.flags = flags;
    }

    @Override
    public String toString() {
        return "BatteryStatus{" +
                "level=" + level +
                ", flags=" + flags +
                ", location=" + location +
                '}';
    }
}
