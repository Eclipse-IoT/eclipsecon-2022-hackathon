package io.drogue.iot.hackathon.ui;

import javax.inject.Inject;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import io.drogue.iot.hackathon.DeviceCommand;
import org.eclipse.microprofile.reactive.messaging.Channel;

import io.drogue.iot.hackathon.Processor;
import io.drogue.iot.hackathon.data.DeviceEvent;
import io.smallrye.mutiny.Multi;
import org.jboss.resteasy.reactive.RestStreamElementType;

/**
 * This resource is used by the main UI entrypoint, providing a stream for the events.
 */
@Path("/events")
public class EventsResource {

    @Inject
    Processor processor;

    @Inject
    @Channel("display-changes")
    Multi<DisplaySettings> displayChanges;

    @Inject
    @Channel("event-stream")
    Multi<DeviceEvent> events;

    @Inject
    @Channel("device-commands")
    Multi<DeviceCommand> commands;

    @GET
    @Path("/stream")
    @Produces(MediaType.SERVER_SENT_EVENTS)
    @RestStreamElementType(MediaType.APPLICATION_JSON)
    public Multi<DeviceEvent> stream() {
        return this.events;
    }

    @GET
    @Path("/display")
    @Produces(MediaType.SERVER_SENT_EVENTS)
    @RestStreamElementType(MediaType.APPLICATION_JSON)
    public Multi<DisplaySettings> displayChanges() {
        return Multi.createFrom()
                .item(this.processor.getDisplaySettings())
                .onCompletion().switchTo(this.displayChanges);
    }

    @GET
    @Path("/commands")
    @Produces(MediaType.SERVER_SENT_EVENTS)
    @RestStreamElementType(MediaType.APPLICATION_JSON)
    public Multi<DeviceCommand> commands() {
        return this.commands;
    }

}
