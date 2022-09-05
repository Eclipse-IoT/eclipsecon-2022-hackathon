package io.drogue.iot.hackathon.registry;

import org.eclipse.microprofile.rest.client.annotation.RegisterClientHeaders;
import org.eclipse.microprofile.rest.client.inject.RegisterRestClient;

import javax.ws.rs.Consumes;
import javax.ws.rs.GET;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.PathParam;
import javax.ws.rs.Produces;
import javax.ws.rs.QueryParam;
import javax.ws.rs.core.MediaType;
import java.util.List;

@Path("/api/registry/v1alpha1")
@RegisterRestClient
@RegisterClientHeaders(RegisterAuthHeaderFactory.class)
public interface RegistryService {
    @GET
    @Path("/apps/{application}/devices")
    @Produces(MediaType.APPLICATION_JSON)
    List<Device> getDevices(@PathParam("application") String application, @QueryParam("labels") String labelSelector);

    @POST
    @Path("/apps/{application}/devices")
    @Consumes(MediaType.APPLICATION_JSON)
    void createDevice(@PathParam("application") String application, Device device);
}
