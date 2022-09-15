package io.drogue.iot.hackathon.endpoints;

import static io.drogue.iot.hackathon.StateHolder.UPDATES;

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
import javax.websocket.OnOpen;
import javax.websocket.Session;
import javax.websocket.server.ServerEndpoint;

import org.eclipse.microprofile.reactive.messaging.Incoming;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.hackathon.StateHolder;
import io.quarkus.qute.CheckedTemplate;
import io.quarkus.qute.TemplateInstance;
import io.quarkus.scheduler.Scheduled;

@ServerEndpoint("/api/updates/v1alpha1/events")
@ApplicationScoped
public class Updates {

    private static final Logger logger = LoggerFactory.getLogger(Updates.class);

    @CheckedTemplate
    public static class Templates {
        public static native TemplateInstance state(StateHolder.State state);
    }

    private final Map<String, Session> sessions = new ConcurrentHashMap<>();

    @Inject
    StateHolder state;

    private Instant lastUpdate = Instant.now();

    @Incoming(UPDATES)
    void update(StateHolder.State state) {
        logger.debug("State update: {}", state);
        var renderedState = Templates.state(state).render();
        logger.trace("Rendered: {}", renderedState);
        logger.debug("Broadcasting to {} sessions", this.sessions.size());
        this.lastUpdate = Instant.now();
        for (var session : this.sessions.values()) {
            session.getAsyncRemote().sendText(renderedState);
        }
    }

    @OnOpen
    void onOpen(Session session) {
        logger.info("onOpen[{}]", session.getId());
        addSession(session);
    }

    @OnClose
    void onClose(Session session) {
        logger.info("onClose[{}]", session.getId());
        removeSession(session);
    }

    @OnError
    void onError(Session session, Throwable error) {
        logger.info("onError[{}]", session.getId(), error);
        removeSession(session);
    }

    void addSession(Session session) {
        var renderedState = Templates.state(this.state.getState()).render();
        session.getAsyncRemote().sendText(renderedState);
        this.sessions.put(session.getId(), session);
    }

    void removeSession(Session session) {
        this.sessions.remove(session.getId());
        try {
            session.close();
        } catch (IOException e) {
            logger.info("Failed to close session ({})", session.getId(), e);
        }
    }

    @Scheduled(every = "10s")
    void ping() {
        if (Duration.between(this.lastUpdate, Instant.now()).getSeconds() > 60) {
            this.lastUpdate = Instant.now();
            var payload = ByteBuffer.allocate(0);
            for (var session : this.sessions.values()) {
                try {
                    session.getAsyncRemote().sendPing(payload);
                } catch (Exception e) {
                    logger.info("Failed to ping session ({})", session.getId(), e);
                    removeSession(session);
                }
            }
        }
    }

}
