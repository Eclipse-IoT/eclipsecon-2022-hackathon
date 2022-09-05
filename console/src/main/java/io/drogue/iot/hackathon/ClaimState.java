package io.drogue.iot.hackathon;

import javax.enterprise.context.ApplicationScoped;

// State of claims per user.
// TODO: Replace with the real thing
@ApplicationScoped
public class ClaimState {
    private volatile ClaimStatus current = new ClaimStatus(false, "");


    public void update(ClaimStatus status) {
        this.current = status;
    }

    public ClaimStatus getCurrent() {
        return this.current;
    }
}
