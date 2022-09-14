package io.drogue.iot.hackathon.endpoints;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.ws.rs.Consumes;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import org.eclipse.microprofile.config.inject.ConfigProperty;

import io.quarkus.runtime.annotations.RegisterForReflection;

@Path("/.well-known/eclipsecon-2022/endpoints")
@Consumes()
@Produces(MediaType.APPLICATION_JSON)
public class EndpointsResource {

    @RegisterForReflection
    @ApplicationScoped
    public static class Endpoints {
        @ConfigProperty(name = "quarkus.oidc.auth-server-url")
        String authServerUrl;

        @ConfigProperty(name = "console.simulatorUrl")
        String simulatorUrl;

        public String getAuthServerUrl() {
            return this.authServerUrl;
        }

        public String getSimulatorUrl() {
            return this.simulatorUrl;
        }
    }

    @Inject
    Endpoints endpoints;

    @GET
    public Endpoints getEndpoints() {
        return this.endpoints;
    }

}
