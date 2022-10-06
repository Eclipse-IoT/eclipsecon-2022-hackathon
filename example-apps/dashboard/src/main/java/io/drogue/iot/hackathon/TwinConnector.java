package io.drogue.iot.hackathon;

import java.net.URI;
import java.time.Duration;
import java.util.Collections;
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

import com.google.common.collect.ImmutableMap;

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
import io.vertx.mutiny.core.http.HttpClient;
import io.vertx.mutiny.core.http.WebSocket;

/**
 * A connector to the twin notifications.
 * <p>
 * This connector establishes a websocket connection to the twin backend, and keeps it open.
 * It also listens to the announces "things" of the mesh, and subscribes to their data.
 */
@Startup
@ApplicationScoped
public class TwinConnector {
    private static final Logger logger = LoggerFactory.getLogger(TwinConnector.class);

    /**
     * The twin application name.
     */
    @ConfigProperty(name = "drogue.application")
    String application;

    /**
     * The base API URL of the twin backend.
     */
    @ConfigProperty(name = "drogue.doppelgaenger.api")
    URI apiUrl;

    /**
     * The root thing of the bluetooth mesh.
     */
    @ConfigProperty(name = "drogue.doppelgaenger.rootId")
    String rootId;

    /**
     * The client for retrieving fresh OAuth2 tokens for the Twin websocket connection.
     */
    @Inject
    OidcClient client;

    @Inject
    Vertx vertx;

    /**
     * The holder of the state.
     * <p>
     * This instance will receive the state updates from the twin connection.
     */
    @Inject
    StateHolder stateHolder;

    private final Map<String, Map<String, BasicFeature>> values = new HashMap<>();

    private Cancellable connecting;

    private volatile boolean stopped;

    private HttpClient httpClient;

    private WebSocket ws;

    @PostConstruct
    public void start() {
        connect();

        final var secure = this.apiUrl.getScheme().equals("https");
        this.httpClient = this.vertx
                .createHttpClient(new HttpClientOptions()
                        .setSsl(secure));
    }

    @PreDestroy
    public void stop() {
        this.stopped = true;
        if (this.connecting != null) {
            this.connecting.cancel();
            this.connecting = null;
        }
        if (this.ws != null) {
            this.ws.closeAndForget();
            this.ws = null;
        }
        if (this.httpClient != null) {
            this.httpClient.closeAndForget();
            this.httpClient = null;
        }
    }

    /**
     * Perform a new connection attempt.
     *
     * This will fetch a new OAuth2 token, and then try to connect to the websocket endpoint.
     * If this succeeds, it will start processing, otherwise it will try to reconnect.
     */
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

                    return this.httpClient
                            .webSocket(port, host, uri.toString());

                })

                .subscribe()
                .with(this::connected, this::failed);
    }

    /**
     * Called when the websocket connection was established.
     *
     * @param webSocket The connected websocket.
     */
    private void connected(final WebSocket webSocket) {
        logger.info("Connected");
        webSocket
                .textMessageHandler(this::onMessage)
                .closeHandler(this::closed);
        this.ws = webSocket;
        this.connecting = null;

        subscribe(this.rootId);
    }

    /**
     * Called when the websocket got closed.
     */
    private void closed() {
        logger.info("Connection closed by remote");
        disconnected();
    }

    /**
     * Called when the websocket failed.
     */
    private void failed(final Throwable throwable) {
        logger.info("Connect failed", throwable);
        this.connecting = null;
        disconnected();
    }

    /**
     * Handle getting disconnected.
     */
    private void disconnected() {
        this.ws = null;
        // clear known children
        this.values.clear();
        // send state
        sendState();
        // trigger reconnect
        reconnect();
    }

    /**
     * Handle a re-connect situation.
     *
     * This method will only schedule a re-connect if the connector is still active.
     */
    private void reconnect() {
        logger.info("Checking reconnect");
        if (this.connecting == null && !this.stopped) {
            logger.info("Scheduling reconnect");
            this.connecting = Uni.createFrom()
                    .item(new Object())
                    .onItem().delayIt().by(Duration.ofSeconds(5))
                    .subscribe().with(x -> connect());
        }
    }

    /**
     * Process incoming messages.
     *
     * @param message The message to process.
     */
    private void onMessage(final String message) {
        logger.debug("onMessage: {}", message);
        final var json = new JsonObject(message);
        final var type = json.getString("type");

        try {
            if ("change".equals(type)) {
                final var thing = json.getJsonObject("thing").mapTo(Thing.class);
                logger.debug("Update: {}", thing);
                thingUpdate(thing);
            } else if ("initial".equals(type)) {
                final var thing = json.getJsonObject("thing").mapTo(Thing.class);
                logger.debug("Initial update: {}", thing);
                thingUpdate(thing);
            }
        } catch (final Exception e) {
            logger.info("Failed to handle message", e);
        }
    }

    /**
     * Send a request to subscribe to a thing.
     *
     * @param thingId The thing to unsubscribe from.
     */
    void subscribe(final String thingId) {
        final var r = new ThingRequest();
        r.type = ThingRequestType.Subscribe;
        r.thing = thingId;
        send(Json.encode(r));
    }

    /**
     * Send a request to unsubscribe from a thing.
     *
     * @param thingId The thing to unsubscribe from.
     */
    void unsubscribe(final String thingId) {
        final var r = new ThingRequest();
        r.type = ThingRequestType.Unsubscribe;
        r.thing = thingId;
        send(Json.encode(r));
    }

    /**
     * Send a text message to the server side.
     *
     * @param text The text to send.
     */
    void send(final String text) {
        final var ws = this.ws;
        if (ws != null) {
            ws.writeTextMessageAndForget(text);
        }
    }

    private void thingUpdate(final Thing thing) throws Exception {
        if (this.rootId.equals(thing.metadata.name)) {
            setRoot(Optional.ofNullable(thing.reportedState.get("$children"))
                    .map(r -> r.getValue())
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

        sendState();
    }

    private void sendState() {
        final var values = new HashMap<String, Map<String, BasicFeature>>(this.values.size());
        for (final var device : this.values.entrySet()) {
            values.put(device.getKey(), ImmutableMap.copyOf(device.getValue()));
        }
        this.stateHolder.setState(Collections.unmodifiableMap(values));
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

        sendState();
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
