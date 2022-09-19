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
        Session session;

        int sortBy;

        Direction direction;

        Connection(final Session session) {
            this.session = session;
        }

        void sendRenderedState(final StateHolder.State state) {
            final var renderedState = renderState(state, this.sortBy, this.direction);
            this.session.getAsyncRemote().sendText(renderedState);
        }
    }

    static String renderState(final StateHolder.State state, final int sortBy, final Direction direction) {

        final var table = new Table("Device ID", "Temperature", "Noise", "Acceleration", "Battery");
        for (final var entry : state.getDevices().entrySet()) {
            final var values = entry.getValue();
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

        if (sortBy >= 0) {
            table.sortBy(sortBy, direction);
        }

        return Templates.state(table).render();
    }

    private final Map<String, Connection> connections = new ConcurrentHashMap<>();

    @Inject
    StateHolder state;

    private Instant lastUpdate = Instant.now();

    @Incoming(UPDATES)
    void update(final StateHolder.State state) {
        logger.debug("State update: {}", state);
        this.lastUpdate = Instant.now();

        logger.debug("Broadcasting to {} sessions", this.connections.size());
        for (final var connection : this.connections.values()) {
            connection.sendRenderedState(state);
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
        logger.info("onMessage - msg: {}", msg);
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
                    connection.sendRenderedState(this.state.getState());
                }
            }
        }
    }

    void addSession(final Session session) {
        final var connection = new Connection(session);
        final var lastState = this.state.getState();
        connection.sendRenderedState(lastState);
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

    @Scheduled(every = "10s")
    void ping() {
        if (Duration.between(this.lastUpdate, Instant.now()).getSeconds() > 60) {
            this.lastUpdate = Instant.now();
            final var payload = ByteBuffer.allocate(0);
            for (final var connection : this.connections.values()) {
                try {
                    connection.session.getAsyncRemote().sendPing(payload);
                } catch (final Exception e) {
                    logger.info("Failed to ping session ({})", connection.session.getId(), e);
                    removeSession(connection.session);
                }
            }
        }
    }

}
