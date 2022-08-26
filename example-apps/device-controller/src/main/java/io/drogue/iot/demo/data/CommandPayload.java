package io.drogue.iot.demo.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

/**
 * An outgoing device message.
 */
@RegisterForReflection
public class CommandPayload {
    private final OnOffSet display;

    public CommandPayload(OnOffSet display) {
        this.display = display;
    }

    public OnOffSet getDisplay() {
        return display;
    }

    @Override
    public String toString() {
        return "DeviceCommand{" +
                "display=" + display +
                '}';
    }
}
