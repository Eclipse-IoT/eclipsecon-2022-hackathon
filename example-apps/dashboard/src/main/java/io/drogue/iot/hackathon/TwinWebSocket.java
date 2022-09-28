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

import io.drogue.iot.hackathon.model.BasicFeature;
import io.drogue.iot.hackathon.model.Thing;
import io.drogue.iot.hackathon.model.ThingRequest;
import io.drogue.iot.hackathon.model.ThingRequestType;
import io.vertx.core.json.Json;
import io.vertx.core.json.JsonObject;

/**
 * Twin connected based on Jakarta WebSockets.
 *
 * @deprecated Unfortunately Jakarta web socket clients seems to have no great story around re-connecting.\
 *         Therefore, we use vert.x for implementing the functionality. See {@link TwinConnector}.
 */
@ClientEndpoint
@Deprecated
public class TwinWebSocket {
    private static final Logger logger = LoggerFactory.getLogger(TwinWebSocket.class);

    @ConfigProperty(name = "drogue.doppelgaenger.rootId")
    String rootId;

    private Session session;

    private final Map<String, Map<String, BasicFeature>> values = new HashMap<>();

    @Inject
    StateHolder stateHolder;

    @OnOpen
    public void onOpen(final Session session) throws Exception {
        logger.info("Connected");
        this.session = session;

        subscribe(this.rootId);
    }

    @OnClose
    public void onClose(final CloseReason reason) {
        logger.info("Closed: {}", reason);
    }

    public void subscribe(final String thingId) throws Exception {
        final var r = new ThingRequest();
        r.type = ThingRequestType.Subscribe;
        r.thing = thingId;
        this.session.getAsyncRemote().sendText(Json.encode(r));
    }

    public void unsubscribe(final String thingId) throws Exception {
        final var r = new ThingRequest();
        r.type = ThingRequestType.Unsubscribe;
        r.thing = thingId;
        this.session.getAsyncRemote().sendText(Json.encode(r));
    }

    @OnMessage
    public void onMessage(final String message) throws Exception {
        logger.info("onMessage: {}", message);
        final var json = new JsonObject(message);
        final var type = json.getString("type");

        if ("change".equals(type)) {
            final var thing = json.getJsonObject("thing").mapTo(Thing.class);
            logger.info("Update: {}", thing);
            thingUpdate(thing);
        } else if ("initial".equals(type)) {
            final var thing = json.getJsonObject("thing").mapTo(Thing.class);
            logger.info("Initial update: {}", thing);
            thingUpdate(thing);
        }
    }

    private void thingUpdate(final Thing thing) throws Exception {
        if (this.rootId.equals(thing.metadata.name)) {
            setRoot(Optional.ofNullable(thing.reportedState.get("$children"))
                    .map(BasicFeature::getValue)
                    .filter(Map.class::isInstance)
                    .map(Map.class::cast)
                    .map(Map::keySet)
                    .orElseGet(Set::of)
            );
        } else {
            setState(thing);
        }
    }

    private void setState(final Thing thing) {
        final var name = thing.metadata.name;
        if (!this.values.containsKey(name)) {
            return;
        }

        final var values = new HashMap<String, BasicFeature>();
        values.putAll(thing.reportedState);
        values.putAll(thing.syntheticState);
        this.values.put(name, values);

        this.stateHolder.setState(this.values);
    }

    @SuppressWarnings("rawtypes")
    private void setRoot(final Set children) throws Exception {
        logger.info("Root: {}", children);

        final var current = new HashSet<>(this.values.keySet());

        for (final var child : children) {
            final var childId = child.toString() + "/sensor";
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

    private void addChild(final String thingId) throws Exception {
        logger.info("Add child: {}", thingId);
        subscribe(thingId);
    }

    private void removeChild(final String thingId) throws Exception {
        logger.info("Remove child: {}", thingId);
        unsubscribe(thingId);
    }

    public Map<String, Map<String, BasicFeature>> getValues() {
        return this.values;
    }
}
