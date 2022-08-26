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

@Path("/commands")
public class CommandsResource {
    @Inject
    Processor processor;

    @POST
    @Path("/display")
    @Consumes(MediaType.APPLICATION_JSON)
    public void updateDisplay(DisplaySettings settings) {
        this.processor.updateDisplaySettings(settings);
    }

}
