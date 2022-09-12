package io.drogue.iot.hackathon.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

/**
 * An outgoing device message.
 */
@RegisterForReflection
public class CommandPayload {
    private final Long address;
    private final OnOffSet display;

    public CommandPayload(Long address, OnOffSet display) {
        this.address = address;
        this.display = display;
    }

    public OnOffSet getDisplay() {
        return display;
    }

    public Long getAddress() {
        return address;
    }

    @Override
    public String toString() {
        return "CommandPayload{" +
                "address=" + address +
                ", display=" + display +
                '}';
    }
}
