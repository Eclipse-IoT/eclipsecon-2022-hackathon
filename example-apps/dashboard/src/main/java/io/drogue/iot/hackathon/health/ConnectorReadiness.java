package io.drogue.iot.hackathon.health;

import javax.inject.Inject;

import org.eclipse.microprofile.health.HealthCheck;
import org.eclipse.microprofile.health.HealthCheckResponse;
import org.eclipse.microprofile.health.HealthCheckResponseBuilder;
import org.eclipse.microprofile.health.Readiness;

import io.drogue.iot.hackathon.TwinConnector;

@Readiness
public class ConnectorReadiness implements HealthCheck {

    @Inject
    TwinConnector twinConnector;

    @Override
    public HealthCheckResponse call() {
        HealthCheckResponseBuilder responseBuilder = HealthCheckResponse.named("Twin WebSocket Connector");

        return responseBuilder
                .status(this.twinConnector.isConnected())
                .build();
    }
}
