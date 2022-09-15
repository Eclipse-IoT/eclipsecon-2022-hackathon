package io.drogue.iot.hackathon.data;

public class BatteryFlags {
    private String presence;

    public String getPresence() {
        return presence;
    }

    public void setPresence(String presence) {
        this.presence = presence;
    }

    @Override
    public String toString() {
        return "BatteryFlags{" +
                "presence='" + presence + '\'' +
                '}';
    }
}
