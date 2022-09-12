package io.drogue.iot.hackathon.registry;

import java.util.List;

import javax.ws.rs.Consumes;
import javax.ws.rs.DELETE;
import javax.ws.rs.GET;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.PathParam;
import javax.ws.rs.Produces;
import javax.ws.rs.QueryParam;
import javax.ws.rs.core.MediaType;

import org.eclipse.microprofile.rest.client.annotation.RegisterClientHeaders;
import org.eclipse.microprofile.rest.client.inject.RegisterRestClient;

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

    @DELETE
    @Path("/apps/{application}/devices/{device}")
    void deleteDevice(@PathParam("application") String application, @PathParam("device") String device);

    @GET
    @Path("/apps/{application}/devices/{device}")
    @Produces(MediaType.APPLICATION_JSON)
    Device getDevice(@PathParam("application") String application, @PathParam("device") String device);
}
