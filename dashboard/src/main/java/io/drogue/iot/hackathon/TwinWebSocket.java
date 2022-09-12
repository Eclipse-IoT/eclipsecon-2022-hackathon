package io.drogue.iot.hackathon;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Optional;
import java.util.Set;

import javax.inject.Inject;
import javax.websocket.ClientEndpoint;
import javax.websocket.CloseReason;
import javax.websocket.OnClose;
import javax.websocket.OnMessage;
import javax.websocket.OnOpen;
import javax.websocket.Session;

import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.fasterxml.jackson.annotation.JsonProperty;

import io.drogue.iot.hackathon.model.BasicFeature;
import io.drogue.iot.hackathon.model.Thing;
import io.quarkus.runtime.annotations.RegisterForReflection;
import io.vertx.core.json.Json;
import io.vertx.core.json.JsonObject;

@ClientEndpoint
public class TwinWebSocket {
    private static final Logger logger = LoggerFactory.getLogger(TwinWebSocket.class);

    enum Type {
        @JsonProperty("subscribe")
        Subscribe,
        @JsonProperty("unsubscribe")
        Unsubscribe
    }

    @RegisterForReflection
    static class ThingRequest {
        public Type type;

        public String thing;
    }

    @ConfigProperty(name = "drogue.doppelgaenger.rootId")
    String rootId;

    private Session session;

    private final Map<String, Map<String, BasicFeature>> values = new HashMap<>();

    @Inject
    StateHolder stateHolder;

    @OnOpen
    public void onOpen(Session session) throws Exception {
        logger.info("Connected");
        this.session = session;

        subscribe(this.rootId);
    }

    @OnClose
    public void onClose(CloseReason reason) {
        logger.info("Closed: {}", reason);
    }

    public void subscribe(String thingId) throws Exception {
        var r = new ThingRequest();
        r.type = Type.Subscribe;
        r.thing = thingId;
        this.session.getAsyncRemote().sendText(Json.encode(r));
    }

    public void unsubscribe(String thingId) throws Exception {
        var r = new ThingRequest();
        r.type = Type.Unsubscribe;
        r.thing = thingId;
        this.session.getAsyncRemote().sendText(Json.encode(r));
    }

    @OnMessage
    public void onMessage(String message) throws Exception {
        logger.info("onMessage: {}", message);
        var json = new JsonObject(message);
        var type = json.getString("type");

        if ("change".equals(type)) {
            var thing = json.getJsonObject("thing").mapTo(Thing.class);
            logger.info("Update: {}", thing);
            thingUpdate(thing);
        } else if ("initial".equals(type)) {
            var thing = json.getJsonObject("thing").mapTo(Thing.class);
            logger.info("Initial update: {}", thing);
            thingUpdate(thing);
        }
    }

    private void thingUpdate(Thing thing) throws Exception {
        if (this.rootId.equals(thing.metadata.name)) {
            setRoot(Optional.ofNullable(thing.reportedState.get("$children"))
                    .map(r -> r.value)
                    .filter(Map.class::isInstance)
                    .map(Map.class::cast)
                    .map(Map::keySet)
                    .orElseGet(Set::of)
            );
        } else {
            setState(thing);
        }
    }

    private void setState(Thing thing) {
        var name = thing.metadata.name;
        if (!this.values.containsKey(name)) {
            return;
        }

        var values = new HashMap<String, BasicFeature>();
        values.putAll(thing.reportedState);
        values.putAll(thing.syntheticState);
        this.values.put(name, values);

        this.stateHolder.setState(this.values);
    }

    @SuppressWarnings("rawtypes")
    private void setRoot(Set children) throws Exception {
        logger.info("Root: {}", children);

        var current = new HashSet<>(this.values.keySet());

        for (var child : children) {
            var childId = child.toString() + "/sensor";
            if (!this.values.containsKey(childId)) {
                this.values.put(childId, Map.of());
                addChild(childId);
            } else {
                current.remove(childId);
            }
        }

        // remove all remaining
        for (var remove : current) {
            remove = remove + "/sensor";
            removeChild(remove);
            this.values.remove(remove);
        }

        this.stateHolder.setState(this.values);
    }

    private void addChild(String thingId) throws Exception {
        logger.info("Add child: {}", thingId);
        subscribe(thingId);
    }

    private void removeChild(String thingId) throws Exception {
        logger.info("Remove child: {}", thingId);
        unsubscribe(thingId);
    }

    public Map<String, Map<String, BasicFeature>> getValues() {
        return this.values;
    }
}
