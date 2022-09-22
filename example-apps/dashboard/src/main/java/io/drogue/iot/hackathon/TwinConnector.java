package io.drogue.iot.hackathon;

import java.net.URI;
import java.time.Duration;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Optional;
import java.util.Set;

import javax.annotation.PostConstruct;
import javax.annotation.PreDestroy;
import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.ws.rs.core.UriBuilder;

import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.hackathon.model.BasicFeature;
import io.drogue.iot.hackathon.model.Thing;
import io.drogue.iot.hackathon.model.ThingRequest;
import io.drogue.iot.hackathon.model.ThingRequestType;
import io.quarkus.oidc.client.OidcClient;
import io.quarkus.runtime.Startup;
import io.smallrye.mutiny.Uni;
import io.smallrye.mutiny.subscription.Cancellable;
import io.vertx.core.http.HttpClientOptions;
import io.vertx.core.json.Json;
import io.vertx.core.json.JsonObject;
import io.vertx.mutiny.core.Vertx;
import io.vertx.mutiny.core.http.WebSocket;

@Startup
@ApplicationScoped
public class TwinConnector {
    private static final Logger logger = LoggerFactory.getLogger(TwinConnector.class);

    @ConfigProperty(name = "drogue.application")
    String application;

    @ConfigProperty(name = "drogue.doppelgaenger.api")
    URI apiUrl;

    @ConfigProperty(name = "drogue.doppelgaenger.rootId")
    String rootId;

    @Inject
    OidcClient client;

    @Inject
    Vertx vertx;

    @Inject
    StateHolder stateHolder;

    private final Map<String, Map<String, BasicFeature>> values = new HashMap<>();

    private Cancellable connecting;

    private volatile boolean stopped;

    private WebSocket ws;

    @PostConstruct
    public void start() {
        connect();
    }

    private void connect() {
        logger.info("Connecting websocket");
        if (this.stopped) {
            return;
        }

        this.connecting = this.client.getTokens()
                .flatMap(tokens -> {

                    final var secure = this.apiUrl.getScheme().equals("https");
                    final var uri = UriBuilder.fromUri(this.apiUrl)
                            .scheme(secure ? "wss" : "ws")
                            .path("/api/v1alpha1/things/{application}/notifications")
                            .queryParam("token", tokens.getAccessToken())
                            .build(this.application);

                    final var host = uri.getHost();
                    var port = uri.getPort();

                    if (port <= 0) {
                        port = secure ? 443 : 80;
                    }

                    return this.vertx
                            .createHttpClient(new HttpClientOptions()
                                    .setSsl(secure))
                            .webSocket(port, host, uri.toString());

                })

                .subscribe()
                .with(this::connected, this::failed);
    }

    private void connected(final WebSocket webSocket) {
        logger.info("Connected");
        webSocket
                .textMessageHandler(this::onMessage)
                .closeHandler(this::closed);
        this.ws = webSocket;
        this.connecting = null;

        subscribe(this.rootId);
    }

    private void closed() {
        logger.info("Connection closed by remote");
        this.ws = null;
        this.stateHolder.setState(Map.of());
        reconnect();
    }

    private void failed(final Throwable throwable) {
        logger.info("Connect failed", throwable);
        this.ws = null;
        this.connecting = null;
        this.stateHolder.setState(Map.of());
        reconnect();
    }

    private void reconnect() {
        logger.info("Checking reconnect");
        if (this.connecting == null && !this.stopped) {
            logger.info("Scheduling reconnect");
            this.connecting = Uni.createFrom()
                    .item(new Object())
                    .onItem().delayIt().by(Duration.ofSeconds(5))
                    .subscribe().with(x -> this.connect());
        }
    }

    @PreDestroy
    public void stop() {
        this.stopped = true;
        if (this.connecting != null) {
            this.connecting.cancel();
        }
        if (this.ws != null) {
            this.ws.closeAndForget();
        }

    }

    private void onMessage(final String message) {
        logger.info("onMessage: {}", message);
        final var json = new JsonObject(message);
        final var type = json.getString("type");

        try {
            if ("change".equals(type)) {
                final var thing = json.getJsonObject("thing").mapTo(Thing.class);
                logger.info("Update: {}", thing);
                thingUpdate(thing);
            } else if ("initial".equals(type)) {
                final var thing = json.getJsonObject("thing").mapTo(Thing.class);
                logger.info("Initial update: {}", thing);
                thingUpdate(thing);
            }
        } catch (final Exception e) {
            logger.info("Failed to handle message", e);
        }
    }

    void subscribe(final String thingId) {
        final var r = new ThingRequest();
        r.type = ThingRequestType.Subscribe;
        r.thing = thingId;
        send(Json.encode(r));
    }

    void unsubscribe(final String thingId) {
        final var r = new ThingRequest();
        r.type = ThingRequestType.Unsubscribe;
        r.thing = thingId;
        send(Json.encode(r));
    }

    void send(final String text) {
        final var ws = this.ws;
        if (ws != null) {
            ws.writeTextMessageAndForget(text);
        }
    }

    private void thingUpdate(final Thing thing) throws Exception {
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
    private void setRoot(final Set children) {
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
        for (final var remove : current) {
            removeChild(remove);
            this.values.remove(remove);
        }

        this.stateHolder.setState(this.values);
    }

    private void addChild(final String thingId) {
        logger.info("Add child: {}", thingId);
        subscribe(thingId);
    }

    private void removeChild(final String thingId) {
        logger.info("Remove child: {}", thingId);
        unsubscribe(thingId);
    }

    public boolean isConnected() {
        return this.connecting == null;
    }
}
