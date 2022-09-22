package io.drogue.iot.hackathon.events;

import java.time.Duration;
import java.util.Map;

import javax.websocket.CloseReason;
import javax.websocket.Session;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.hackathon.utils.DummyRoutingContext;
import io.quarkus.oidc.AccessTokenCredential;
import io.quarkus.security.identity.IdentityProviderManager;
import io.quarkus.security.identity.SecurityIdentity;
import io.quarkus.security.identity.request.TokenAuthenticationRequest;
import io.quarkus.vertx.http.runtime.security.HttpSecurityUtils;
import io.smallrye.mutiny.Multi;
import io.smallrye.mutiny.infrastructure.Infrastructure;
import io.smallrye.mutiny.subscription.Cancellable;
import io.vertx.core.json.Json;
import io.vertx.core.json.JsonObject;

public class EventSession implements AutoCloseable {

    private static final Logger logger = LoggerFactory.getLogger(EventSession.class);

    private final IdentityProviderManager identityProviderManager;

    private final Session session;

    private final EventDispatcher.DispatcherContext context;

    private Cancellable ticker;

    private SecurityIdentity identity;

    private Cancellable subscription;

    public EventSession(
            EventDispatcher.DispatcherContext context, IdentityProviderManager identityProviderManager, Session session) {
        this.identityProviderManager = identityProviderManager;
        this.context = context;
        this.session = session;
        this.ticker = Multi.createFrom().ticks().every(Duration.ofSeconds(15)).subscribe().with(this::tick);
    }

    void tick(Long ignore) {
        var identity = this.identity;
        if (identity != null) {
            var expiry = identity.getAttribute("quarkus.identity.expire-time");
            if (expiry instanceof Number) {
                if ((System.currentTimeMillis() / 1000) > ((Number) expiry).intValue()) {
                    this.context.close(new CloseReason(CloseReason.CloseCodes.VIOLATED_POLICY, "Token expired without submitting a new one"));
                }
            }
        }
    }

    /**
     * Called when the session got closed.
     * <p>
     * To close the session, use {@link Session#close(CloseReason)}.
     */
    @Override
    public void close() {
        logger.info("Close session");
        if (this.subscription != null) {
            this.subscription.cancel();
            this.subscription = null;
        }
        if (this.ticker != null) {
            this.ticker.cancel();
            this.ticker = null;
        }
    }

    public void handleMessage(String message) throws Exception {
        logger.info("Handle message: {}", message);
        var json = new JsonObject(message);
        if (json.containsKey("token")) {
            handleAccessToken(json.getString("token"));
        }
    }

    private void handleAccessToken(String accessToken) throws Exception {
        var request = new TokenAuthenticationRequest(new AccessTokenCredential(accessToken));
        HttpSecurityUtils.setRoutingContextAttribute(request, new DummyRoutingContext());
        this.identityProviderManager
                .authenticate(request)
                .runSubscriptionOn(Infrastructure.getDefaultWorkerPool())
                .subscribe()
                .with(this::setIdentity, (error) -> {
                    logger.warn("Failed to validate token", error);
                    this.context.close(new CloseReason(CloseReason.CloseCodes.UNEXPECTED_CONDITION, String.format("Failed to validate token: %s", error.getMessage())));
                });
    }

    private void setIdentity(SecurityIdentity identity) {
        logger.info("Identity: {} ({})", identity.getPrincipal(), identity.getRoles());
        for (Map.Entry<String, Object> entry : identity.getAttributes().entrySet()) {
            logger.info("   {}: {}", entry.getKey(), entry.getValue());
        }
        if (this.identity == null) {
            subscribe(identity);
        } else {
            this.identity = identity;
        }
    }

    private void subscribe(SecurityIdentity identity) {
        this.identity = identity;
        this.subscription = this.context
                .subscribe(identity)
                .subscribe()
                .with(this::handleDeviceState);
    }

    private void handleDeviceState(State state) {
        logger.info("Device state [{}]: {}", this.session.getId(), state);
        this.session.getAsyncRemote()
                .sendText(Json.encode(state));
    }

}
