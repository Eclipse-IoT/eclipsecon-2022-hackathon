package io.drogue.iot.demo.ui;

import javax.inject.Inject;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import org.eclipse.microprofile.reactive.messaging.Channel;
import org.jboss.resteasy.reactive.RestSseElementType;

import io.drogue.iot.demo.Processor;
import io.drogue.iot.demo.data.DeviceCommand;
import io.drogue.iot.demo.data.DeviceEvent;
import io.smallrye.mutiny.Multi;

/**
 * This resource is used by the main UI entrypoint, providing a stream for the events.
 */
@Path("/events")
public class EventsResource {

    @Inject
    Processor processor;

    @Inject
    @Channel("response-changes")
    Multi<String> responseChanges;

    @Inject
    @Channel("event-stream")
    Multi<DeviceEvent> events;

    @Inject
    @Channel("device-commands")
    Multi<DeviceCommand> commands;

    @GET
    @Path("/stream")
    @Produces(MediaType.SERVER_SENT_EVENTS)
    @RestSseElementType(MediaType.APPLICATION_JSON)
    public Multi<DeviceEvent> stream() {
        return this.events;
    }

    @GET
    @Path("/response")
    @Produces(MediaType.SERVER_SENT_EVENTS)
    @RestSseElementType(MediaType.APPLICATION_JSON)
    public Multi<String> responseChanges() {
        return Multi.createFrom()
                .item(this.processor.getResponse())
                .onCompletion().switchTo(this.responseChanges);
    }

    @GET
    @Path("/commands")
    @Produces(MediaType.SERVER_SENT_EVENTS)
    @RestSseElementType(MediaType.APPLICATION_JSON)
    public Multi<DeviceCommand> commands() {
        return this.commands;
    }

}
