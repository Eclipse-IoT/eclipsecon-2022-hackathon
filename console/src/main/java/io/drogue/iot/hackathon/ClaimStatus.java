package io.drogue.iot.hackathon;

public class ClaimStatus {
    private final Boolean claimed;
    private final String device;

    public ClaimStatus(Boolean claimed, String device) {
        this.claimed = claimed;
        this.device = device;
    }

    public Boolean getClaimed() {
        return claimed;
    }

    public String getDevice() {
        return device;
    }
}
