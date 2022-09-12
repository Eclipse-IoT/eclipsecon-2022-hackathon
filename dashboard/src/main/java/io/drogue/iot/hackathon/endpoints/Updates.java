package io.drogue.iot.hackathon.endpoints;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import javax.inject.Inject;
import javax.websocket.OnClose;
import javax.websocket.OnError;
import javax.websocket.OnOpen;
import javax.websocket.Session;
import javax.websocket.server.ServerEndpoint;

import org.eclipse.microprofile.reactive.messaging.Incoming;

import io.drogue.iot.hackathon.StateHolder;
import io.quarkus.qute.CheckedTemplate;
import io.quarkus.qute.TemplateInstance;

@ServerEndpoint("/api/updates/v1alpha1/events")
public class Updates {

    @CheckedTemplate
    public static class Templates {
        public static native TemplateInstance state(StateHolder.State state);
    }

    private final Map<String, Session> sessions = new ConcurrentHashMap<>();

    @Inject
    StateHolder state;

    @Incoming("updates")
    void update(StateHolder.State state) {
        var renderedState = Templates.state(state).render();
        for (var session : this.sessions.values()) {
            session.getAsyncRemote().sendText(renderedState);
        }
    }

    @OnOpen
    void onOpen(Session session) {
        addSession(session);
    }

    @OnClose
    void onClose(Session session) throws IOException {
        removeSession(session);
    }

    @OnError
    void onError(Session session, Throwable error) throws IOException {
        removeSession(session);
    }

    void addSession(Session session) {
        var renderedState = Templates.state(this.state.getState()).render();
        session.getAsyncRemote().sendText(renderedState);
        this.sessions.put(session.getId(), session);
    }

    void removeSession(Session session) throws IOException {
        session = this.sessions.remove(session.getId());
        if (session != null) {
            session.close();
        }
    }
}
