package io.drogue.iot.hackathon.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

/**
 * An outgoing device message.
 */
@RegisterForReflection
public class CommandPayload {
    private final long address;
    private OnOffSet speaker;
    private LevelSet display;

    public CommandPayload(long address) {
        this.address = address;
    }

    public long getAddress() {
        return address;
    }

    public OnOffSet getSpeaker() {
        return speaker;
    }

    public LevelSet getDisplay() {
        return display;
    }

    @Override
    public String toString() {
        return "CommandPayload{" +
                "address=" + address +
                ", speaker=" + speaker +
                ", display=" + display +
                '}';
    }

    public void setSpeaker(OnOffSet speaker) {
        this.speaker = speaker;
    }

    public void setDisplay(LevelSet display) {
        this.display = display;
    }
}
