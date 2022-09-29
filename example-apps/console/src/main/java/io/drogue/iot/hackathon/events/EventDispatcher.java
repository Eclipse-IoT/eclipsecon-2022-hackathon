package io.drogue.iot.hackathon.events;

import java.time.Instant;
import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.locks.ReadWriteLock;
import java.util.concurrent.locks.ReentrantReadWriteLock;

import javax.annotation.PreDestroy;
import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.websocket.CloseReason;
import javax.websocket.Session;

import org.eclipse.microprofile.reactive.messaging.Incoming;
import org.eclipse.microprofile.reactive.messaging.OnOverflow;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.hackathon.data.DeviceEvent;
import io.drogue.iot.hackathon.data.DevicePayload;
import io.drogue.iot.hackathon.service.DeviceClaimService;
import io.quarkus.security.identity.IdentityProviderManager;
import io.quarkus.security.identity.SecurityIdentity;
import io.smallrye.common.constraint.NotNull;
import io.smallrye.mutiny.Multi;
import io.smallrye.mutiny.Uni;
import io.smallrye.mutiny.infrastructure.Infrastructure;
import io.smallrye.mutiny.operators.multi.processors.BroadcastProcessor;

@ApplicationScoped
public class EventDispatcher {
    private static final Logger logger = LoggerFactory.getLogger(EventDispatcher.class);

    private final ReadWriteLock sessionsLock = new ReentrantReadWriteLock();

    private final Map<String, EventSession> sessions = new HashMap<>();

    private final Map<String, State> latestState = new ConcurrentHashMap<>();

    private final Map<String, BroadcastProcessor<State>> subscriptions = new ConcurrentHashMap<>();

    public interface DispatcherContext {
        /**
         * Start listening for events.
         *
         * @param identity The identity subscribing.
         */
        Multi<State> subscribe(SecurityIdentity identity);

        void close(CloseReason closeReason);
    }

    @Inject
    IdentityProviderManager identityProviderManager;

    @Inject
    DeviceClaimService claimService;

    @Incoming("event-stream")
    @OnOverflow(value = OnOverflow.Strategy.LATEST)
    void onMessage(DeviceEvent event) {
        if (event.getDeviceId() == null || event.getPayload() == null || event.getPayload().getState() == null) {
            return;
        }
        broadcast(event.getDeviceId(), event.getPayload(), event.getTimestamp());
    }

    @PreDestroy
    void dispose() {
        for (var broadcast : this.subscriptions.values()) {
            broadcast.onComplete();
        }
        logger.info("Disposed {} subscriptions", this.subscriptions.size());
        this.subscriptions.clear();
    }

    void broadcast(@NotNull String deviceId, DevicePayload payload, Instant lastChange) {
        logger.info("Broadcast payload from {}: {}", deviceId, payload);

        State newState = new State(lastChange, payload.getState());
        if (payload.isPartial()) {
            newState = this.latestState.merge(deviceId, newState, State::merge);
        } else {
            this.latestState.put(deviceId, newState);
        }

        getSubscription(deviceId)
                .onNext(newState);
    }

    BroadcastProcessor<State> getSubscription(String deviceId) {
        return this.subscriptions.computeIfAbsent(deviceId, k -> {
            return BroadcastProcessor.create();
        });
    }

    Multi<State> subscribe(SecurityIdentity identity) {

        return Multi.createFrom()
                .uni(Uni.createFrom()
                        .item(() -> {
                            return this.claimService.getDeviceClaimFor(identity.getPrincipal().getName());
                        })
                        .runSubscriptionOn(Infrastructure.getDefaultWorkerPool())
                )
                .flatMap(claim -> {
                    final Multi<State> stream;

                    logger.info("Subscribe for: {}", claim);
                    if (claim.isPresent()) {
                        var id = claim.get().getId();
                        var firstItem = Multi.createFrom().optional(Optional.ofNullable(this.latestState.get(id)));
                        var broadcast = getSubscription(id);
                        stream = firstItem.onCompletion()
                                .switchTo(broadcast);
                    } else {
                        stream = Multi.createFrom().empty();
                    }

                    return stream;
                });
    }

    public void createSession(Session session) {
        this.sessionsLock.writeLock().lock();
        try {
            this.sessions.put(session.getId(), new EventSession(
                    new DispatcherContext() {
                        @Override
                        public Multi<State> subscribe(SecurityIdentity identity) {
                            return EventDispatcher.this.subscribe(identity);
                        }

                        @Override
                        public void close(CloseReason closeReason) {
                            try {
                                session.close(closeReason);
                            } catch (Exception e) {
                                EventDispatcher.logger.warn("Failed to close session", e);
                            }
                        }
                    },
                    this.identityProviderManager, session));
        } finally {
            this.sessionsLock.writeLock().unlock();
        }
    }

    public void disposeSession(final Session session) {
        this.sessionsLock.writeLock().lock();
        try {
            var eventSession = this.sessions.remove(session.getId());
            if (eventSession != null) {
                eventSession.close();
            }
        } finally {
            this.sessionsLock.writeLock().unlock();
        }
    }

    public void remoteMessage(Session session, String message) throws Exception {
        this.sessionsLock.readLock().lock();
        try {
            var eventSession = this.sessions.get(session.getId());
            if (eventSession != null) {
                eventSession.handleMessage(message);
            }
        } catch (Exception e) {
            logger.info("Failed to handle message", e);
            session.close(new CloseReason(CloseReason.CloseCodes.PROTOCOL_ERROR, "Failed to handle message: " + e.getMessage()));
        } finally {
            this.sessionsLock.readLock().unlock();
        }
    }

    public void releaseDevice(String deviceId) {
        broadcast(deviceId, DevicePayload.empty(), Instant.now());
    }
}
