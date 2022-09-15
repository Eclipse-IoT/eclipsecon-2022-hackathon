package io.drogue.iot.hackathon;

import java.util.Map;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;

import org.eclipse.microprofile.reactive.messaging.Channel;
import org.eclipse.microprofile.reactive.messaging.Emitter;

import com.google.common.base.MoreObjects;

import io.drogue.iot.hackathon.model.BasicFeature;
import io.smallrye.reactive.messaging.annotations.Broadcast;
import io.vertx.core.json.Json;

@ApplicationScoped
public class StateHolder {

    public static final String UPDATES = "updates";

    private final State state = new State();

    public static class State {
        private volatile Map<String, Map<String, BasicFeature>> devices;

        State() {
            this.devices = Map.of();
        }

        public Map<String, Map<String, BasicFeature>> getDevices() {
            return this.devices;
        }

        public String toJson() {
            return Json.encode(this.devices);
        }

        public boolean isEmpty() {
            return this.devices.isEmpty();
        }

        @Override
        public String toString() {
            return MoreObjects.toStringHelper(this)
                    .add("devices", this.devices)
                    .toString();
        }
    }

    @Inject
    @Channel(UPDATES)
    @Broadcast
    Emitter<State> updates;

    public State getState() {
        return this.state;
    }

    public void setState(Map<String, Map<String, BasicFeature>> state) {
        this.state.devices = state;
        this.updates.send(this.state);
    }

    @Override
    public String toString() {
        return MoreObjects.toStringHelper(this)
                .add("state", this.state.devices)
                .toString();
    }
}
