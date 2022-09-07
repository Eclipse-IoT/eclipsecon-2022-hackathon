package io.drogue.iot.hackathon.events;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.websocket.OnClose;
import javax.websocket.OnError;
import javax.websocket.OnMessage;
import javax.websocket.OnOpen;
import javax.websocket.Session;
import javax.websocket.server.ServerEndpoint;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@ServerEndpoint("/api/events/v1alpha1")
@ApplicationScoped
public class WebSocketEndpoint {

    private static final Logger logger = LoggerFactory.getLogger(WebSocketEndpoint.class);

    @Inject
    EventDispatcher dispatcher;

    @OnOpen
    public void onOpen(Session session) {
        logger.info("onOpen - {}", session.getId());
        this.dispatcher.createSession(session);
    }

    @OnClose
    public void onClose(Session session) {
        logger.info("onClose - {}", session.getId());
        this.dispatcher.disposeSession(session);
    }

    @OnError
    public void onError(Session session, Throwable throwable) {
        logger.info("onError - {}", session.getId(), throwable);
        this.dispatcher.disposeSession(session);
    }

    @OnMessage
    public void onMessage(Session session, String message) throws Exception {
        logger.info("onMessage - {}: {}", session.getId(), message);
        this.dispatcher.remoteMessage(session, message);
    }

}
