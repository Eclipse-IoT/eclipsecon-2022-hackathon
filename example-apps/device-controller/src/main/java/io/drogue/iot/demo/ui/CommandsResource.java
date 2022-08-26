package io.drogue.iot.demo.ui;

import javax.inject.Inject;
import javax.ws.rs.Consumes;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.demo.Processor;

@Path("/commands/display")
public class CommandsResource {

    private static final Logger LOG = LoggerFactory.getLogger(CommandsResource.class);

    @Inject
    Processor processor;

    @POST
    @Consumes(MediaType.TEXT_PLAIN)
    @Produces
    public void setResponse() {
        this.processor.toggleDisplay();
    }

}
