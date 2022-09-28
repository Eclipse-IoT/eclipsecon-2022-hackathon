package io.drogue.iot.hackathon.endpoints;

import static io.drogue.iot.hackathon.StateHolder.UPDATES;
import static io.drogue.iot.hackathon.endpoints.Cell.cell;
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
import io.drogue.iot.hackathon.model.BasicFeature;
import io.quarkus.qute.CheckedTemplate;
import io.quarkus.qute.TemplateInstance;
import io.quarkus.scheduler.Scheduled;
import io.vertx.core.json.JsonObject;

@ServerEndpoint("/api/updates/v1alpha1/events")
@ApplicationScoped
public class Updates {

    private static final Logger logger = LoggerFactory.getLogger(Updates.class);

    @CheckedTemplate
    public static class Templates {
        public static native TemplateInstance state(Table table);
    }

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

        private void sendContent() {
            this.needSend = false;
            setNextSend();
            this.session
                    .getAsyncRemote()
                    .sendText(this.lastContent);
        }

        private void setNextSend() {
            this.nextSend = Instant.now().plus(Duration.ofSeconds(1));
        }

        private void sendPing() {
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

    static boolean seemsEmpty(final Map<String, BasicFeature> values) {
        for (final var value : values.entrySet()) {
            if (value.getKey().startsWith("$")) {
                // ignore structural information
                continue;
            }
            if (value.getValue().getValue() != null) {
                return false;
            }
        }
        return true;
    }

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
                                        values.get("battery"))
                                        .flatMap(BasicFeature::toDouble),
                                value -> String.format("%.2f%%", value),
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

    private final Map<String, Connection> connections = new ConcurrentHashMap<>();

    @Inject
    StateHolder state;

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

    void addSession(final Session session) {
        final var connection = new Connection(session);
        final var lastState = this.state.getState();
        connection.sendRenderedState(lastState, false);
        this.connections.put(session.getId(), connection);
    }

    void removeSession(final Session session) {
        this.connections.remove(session.getId());
        try {
            session.close();
        } catch (final IOException e) {
            logger.info("Failed to close session ({})", session.getId(), e);
        }
    }

    @Scheduled(every = "1s")
    void tick() {
        this.connections.values().forEach(Connection::tick);
    }

}
