package io.drogue.iot.hackathon.endpoints;

import static io.drogue.iot.hackathon.StateHolder.UPDATES;
import static io.drogue.iot.hackathon.endpoints.render.Cell.cell;
import static java.util.Optional.ofNullable;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.time.Duration;
import java.time.Instant;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.websocket.OnClose;
import javax.websocket.OnError;
import javax.websocket.OnMessage;
import javax.websocket.OnOpen;
import javax.websocket.Session;
import javax.websocket.server.ServerEndpoint;

import org.eclipse.microprofile.reactive.messaging.Incoming;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.hackathon.StateHolder;
import io.drogue.iot.hackathon.endpoints.render.Direction;
import io.drogue.iot.hackathon.endpoints.render.Table;
import io.drogue.iot.hackathon.model.BasicFeature;
import io.quarkus.qute.CheckedTemplate;
import io.quarkus.qute.TemplateInstance;
import io.quarkus.scheduler.Scheduled;
import io.vertx.core.json.JsonObject;

/**
 * Endpoint for state events.
 * <p>
 * This is a web socket endpoint, which will send out an aggregate view of all things.
 * Each session has its session state and will receive a customized rendering of the current state.
 * <p>
 * It is using the <a href="https://quarkus.io/guides/qute">Qute</a> rendering engine from Quarkus.
 * <p>
 * <h3>Tasks:</h3>
 * <ul>
 *     <li>Cache renderings for sessions with the same state</li>
 * </ul>
 */
@ServerEndpoint("/api/updates/v1alpha1/events")
@ApplicationScoped
public class Events {

    private static final Logger logger = LoggerFactory.getLogger(Events.class);

    /**
     * Binding to the template in {@code resources/templates/Events}
     */
    @CheckedTemplate
    public static class Templates {
        public static native TemplateInstance state(Table table);
    }

    /**
     * A connection (session state) to a listener.
     * <p>
     * The connection will try to limit changes. So only changed which changed the rendered content are sent,
     * as well a limit to not send more often than one second.
     */
    static class Connection {
        private final Session session;

        private int sortBy;

        private Direction direction;

        private Instant nextSend = Instant.now();

        private String lastContent;

        private boolean needSend;

        Connection(final Session session) {
            this.session = session;
        }

        /**
         * Gets ticked every second.
         */
        void tick() {
            if (this.needSend) {
                sendContent();
            } else if (Duration.between(Instant.now(), this.nextSend).toSeconds() > 30) {
                sendPing();
            }

        }

        void sendRenderedState(final StateHolder.State state, final boolean force) {

            final var renderedState = renderState(state, this.sortBy, this.direction);
            if (!renderedState.equals(this.lastContent)) {
                this.lastContent = renderedState;
                if (force || Instant.now().isAfter(this.nextSend)) {
                    sendContent();
                } else {
                    this.needSend = true;
                }
            }

        }

        /**
         * Actually send the content.
         */
        private void sendContent() {
            this.needSend = false;
            setNextSend();
            this.session
                    .getAsyncRemote()
                    .sendText(this.lastContent);
        }

        /**
         * Record the time until the next send.
         */
        private void setNextSend() {
            this.nextSend = Instant.now().plus(Duration.ofSeconds(1));
        }

        /**
         * Send a ping.
         */
        private void sendPing() {
            logger.info("Send ping to: {}", this.session);
            setNextSend();

            try {
                // weird, but the "ping" method of the async client is actually a synchronous call
                this.session
                        .getAsyncRemote()
                        .sendPing(ByteBuffer.wrap(new byte[0]));
            } catch (final Exception e) {
                logger.info("Failed to send ping", e);
            }
        }
    }

    static final String[] EXPECTED_PROPERTIES = new String[] {
            "temperature",
            "batteryLevel",
            "noise",
            "acceleration",
    };

    /**
     * Check if a thing "seems" empty.
     * <p>
     * Things are never really empty, as they contain some technical information.
     * So, this is our definition of "empty" for the end user.
     *
     * @param values The current values of the thing.
     * @return The result.
     */
    static boolean seemsEmpty(final Map<String, BasicFeature> values) {

        for (final var name : EXPECTED_PROPERTIES) {
            final var value = values.get(name);
            if (value == null) {
                continue;
            }
            if (value.getValue() != null) {
                return false;
            }
        }

        return true;
    }

    /**
     * Render the provided state.
     *
     * @param state The state to render.
     * @param sortBy The column to sort by. A negative number indicates to not sort.
     * @param direction The direction to sort by. Must not be {@code null} in case the sortBy field indicates that sorting is required.
     * @return The rendered output.
     */
    static String renderState(final StateHolder.State state, final int sortBy, final Direction direction) {

        final var table = new Table("Device ID", "Temperature", "Noise", "Acceleration", "Battery");
        for (final var entry : state.getDevices().entrySet()) {

            final var values = entry.getValue();

            if (!seemsEmpty(values)) {

                table.addRow(
                        cell(ofNullable(entry.getKey())),
                        cell(ofNullable(values.get("temperature"))
                                        .flatMap(BasicFeature::toDouble),
                                value -> String.format("%.0f °C", value)),
                        cell(ofNullable(values.get("noise"))
                                .flatMap(BasicFeature::toDouble)),
                        cell(ofNullable(values.get("acceleration"))
                                        .flatMap(f -> f.toTyped(Map.class)),
                                value -> String.format("%s / %s / %s", value.get("x"), value.get("y"), value.get("z"))),
                        cell(ofNullable(
                                        values.get("batteryLevel"))
                                        .flatMap(BasicFeature::toDouble),
                                value -> String.format("%.0f %%", value),
                                "N/A")
                );
            }
        }

        if (sortBy >= 0) {
            table.sortBy(sortBy, direction);
        }

        return Templates.state(table)
                .render();
    }

    /**
     * A map of the current connections.
     */
    private final Map<String, Connection> connections = new ConcurrentHashMap<>();

    /**
     * A reference to the current state.
     */
    @Inject
    StateHolder state;

    /**
     * Called when updates get published on the {@link UPDATES} channel.
     *
     * @param state The new state.
     */
    @Incoming(UPDATES)
    void update(final StateHolder.State state) {
        logger.debug("State update: {}", state);

        logger.debug("Broadcasting to {} sessions", this.connections.size());
        for (final var connection : this.connections.values()) {
            connection.sendRenderedState(state, false);
        }
    }

    @OnOpen
    void onOpen(final Session session) {
        logger.debug("onOpen[{}]", session.getId());
        addSession(session);
    }

    @OnClose
    void onClose(final Session session) {
        logger.debug("onClose[{}]", session.getId());
        removeSession(session);
    }

    @OnError
    void onError(final Session session, final Throwable error) {
        logger.info("onError[{}]", session.getId(), error);
        removeSession(session);
    }

    /**
     * Handle messages from the listener.
     *
     * @param session The session which received the message.
     * @param message The received message.
     */
    @OnMessage
    void onMessage(final Session session, final String message) {
        final var msg = new JsonObject(message);
        logger.debug("onMessage - msg: {}", msg);

        if ("sortBy".equals(msg.getString("request"))) {
            final var sortBy = msg.getInteger("column", 0);

            var direction = Direction.ASCENDING;
            try {
                direction = Direction.valueOf(msg.getString("direction", "ascending").toUpperCase());
            } catch (final Exception ignored) {
            }

            if (sortBy != null) {
                final var connection = this.connections.get(session.getId());
                if (connection != null) {
                    connection.sortBy = sortBy;
                    connection.direction = direction;
                    logger.info("Re-sort - sortBy: {}, direction: {}", connection.sortBy, connection.direction);
                    connection.sendRenderedState(this.state.getState(), true);
                }
            }
        }
    }

    /**
     * Add a new session.
     * <p>
     * This will add the session to the internal state, and also send an initial update.
     *
     * @param session The new session to add.
     */
    void addSession(final Session session) {
        final var connection = new Connection(session);
        final var lastState = this.state.getState();
        connection.sendRenderedState(lastState, false);
        this.connections.put(session.getId(), connection);
    }

    /**
     * Remove and close a session.
     *
     * @param session The session to remove.
     */
    void removeSession(final Session session) {
        this.connections.remove(session.getId());
        try {
            session.close();
        } catch (final IOException e) {
            logger.info("Failed to close session ({})", session.getId(), e);
        }
    }

    /**
     * Scheduler, ticking all connections.
     */
    @Scheduled(every = "1s")
    void tick() {
        this.connections.values().forEach(Connection::tick);
    }

}
