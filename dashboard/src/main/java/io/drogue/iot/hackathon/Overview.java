package io.drogue.iot.hackathon;

import java.net.URI;
import java.util.Map;

import javax.annotation.PostConstruct;
import javax.annotation.PreDestroy;
import javax.inject.Inject;
import javax.websocket.CloseReason;
import javax.websocket.ContainerProvider;
import javax.websocket.Session;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.UriBuilder;

import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.hackathon.model.BasicFeature;
import io.quarkus.oidc.client.OidcClient;
import io.quarkus.qute.CheckedTemplate;
import io.quarkus.qute.TemplateInstance;
import io.quarkus.runtime.Startup;
import io.quarkus.scheduler.Scheduled;

@Startup
@Path("/")
public class Overview {
    private static final Logger logger = LoggerFactory.getLogger(TwinConnection.class);

    @ConfigProperty(name = "drogue.application")
    String application;

    @ConfigProperty(name = "drogue.doppelgaenger.api")
    URI apiUrl;

    @Inject
    OidcClient client;

    private Session ws;

    @Inject
    StateHolder state;

    @PostConstruct
    public void start() throws Exception {
        var token = this.client.getTokens().await().indefinitely().getAccessToken();

        logger.info("Scheme: {}", this.apiUrl.getScheme());

        var secure = this.apiUrl.getScheme().equals("https");

        var uri = UriBuilder.fromUri(this.apiUrl)
                .scheme(secure ? "wss" : "ws")
                .path("/api/v1alpha1/things/{application}/notifications")
                .queryParam("token", token)
                .build(this.application);

        logger.info("Connection to WS: {}", uri);
        this.ws = ContainerProvider.getWebSocketContainer()
                .connectToServer(TwinConnection.class, uri);
    }

    @PreDestroy
    public void stop() throws Exception {
        if (this.ws != null) {
            this.ws.close(new CloseReason(CloseReason.CloseCodes.NORMAL_CLOSURE, "Shutting down"));
        }
    }

    @Scheduled(every = "10s")
    public void tick() {
        logger.info("Ticking...");
        logger.info("State: {}", this.state.getState());
    }

    @CheckedTemplate
    public static class Templates {
        public static native TemplateInstance index(Map<String, Map<String, BasicFeature>> state);
    }

    @GET
    @Produces(MediaType.TEXT_HTML)
    public TemplateInstance index() {
        return Templates.index(this.state.getState());
    }
}
