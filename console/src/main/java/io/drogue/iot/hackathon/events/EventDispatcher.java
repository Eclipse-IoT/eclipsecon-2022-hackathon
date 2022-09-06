package io.drogue.iot.hackathon.events;

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
import io.drogue.iot.hackathon.data.DeviceState;
import io.drogue.iot.hackathon.registry.Device;
import io.drogue.iot.hackathon.service.DeviceClaimService;
import io.quarkus.security.identity.IdentityProviderManager;
import io.quarkus.security.identity.SecurityIdentity;
import io.smallrye.mutiny.Multi;
import io.smallrye.mutiny.Uni;
import io.smallrye.mutiny.infrastructure.Infrastructure;
import io.smallrye.mutiny.operators.multi.processors.BroadcastProcessor;

@ApplicationScoped
public class EventDispatcher {
    private static final Logger logger = LoggerFactory.getLogger(EventDispatcher.class);

    private final ReadWriteLock sessionsLock = new ReentrantReadWriteLock();

    private final Map<String, EventSession> sessions = new HashMap<>();

    private final Map<String, DeviceState> latestState = new ConcurrentHashMap<>();

    private final Map<String, BroadcastProcessor<DeviceState>> subscriptions = new ConcurrentHashMap<>();

    public interface DispatcherContext {
        /**
         * Start listening for events.
         *
         * @param identity The identity subscribing.
         */
        Multi<DeviceState> subscribe(SecurityIdentity identity);

        void close(CloseReason closeReason);
    }

    @Inject
    IdentityProviderManager identityProviderManager;

    @Inject
    DeviceClaimService claimService;

    @Incoming("event-stream")
    @OnOverflow(value = OnOverflow.Strategy.LATEST)
    void onMessage(DeviceEvent event) {
        broadcast(event.getDeviceId(), event);
    }

    @PreDestroy
    void dispose() {
        for (var broadcast : this.subscriptions.values()) {
            broadcast.onComplete();
        }
        logger.info("Disposed {} subscriptions", this.subscriptions.size());
        this.subscriptions.clear();
    }

    void broadcast(String deviceId, DeviceEvent event) {
        logger.info("Broadcast event: {}", event);

        final DeviceState newState;
        if (event.getPayload().isPartial()) {
            newState = this.latestState.merge(deviceId, event.getPayload().getState(), DeviceState::merge);
        } else {
            newState = event.getPayload().getState();
            this.latestState.put(deviceId, newState);
        }

        getSubscription(deviceId)
                .onNext(newState);
    }

    BroadcastProcessor<DeviceState> getSubscription(String deviceId) {
        return this.subscriptions.computeIfAbsent(deviceId, k -> {
            return BroadcastProcessor.create();
        });
    }

    Multi<DeviceState> subscribe(SecurityIdentity identity) {

        return Multi.createFrom()
                .uni(Uni.createFrom()
                        .item(() -> {
                            return claimService.getDeviceClaimFor(identity.getPrincipal().getName());
                        })
                        .runSubscriptionOn(Infrastructure.getDefaultWorkerPool())
                )
                .flatMap(claim -> {
                    final Multi<DeviceState> stream;

                    logger.info("Subscribe for: {}", claim);
                    if (claim.isPresent()) {
                        var id = claim.get().id;
                        var firstItem = Multi.createFrom().optional(Optional.ofNullable(latestState.get(id)));
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
                        public Multi<DeviceState> subscribe(SecurityIdentity identity) {
                            return EventDispatcher.this.subscribe(identity);
                        }

                        @Override
                        public void close(CloseReason closeReason) {
                            try {
                                session.close(closeReason);
                            } catch (Exception e) {
                                logger.warn("Failed to close session", e);
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
}
