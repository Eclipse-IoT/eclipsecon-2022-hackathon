package io.drogue.iot.hackathon.ui;

import javax.inject.Inject;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import io.drogue.iot.hackathon.ClaimState;
import io.drogue.iot.hackathon.ClaimStatus;
import io.quarkus.qute.CheckedTemplate;
import io.quarkus.qute.TemplateInstance;

@Path("/")
public class Index {

    @CheckedTemplate
    public static class Templates {
        public static native TemplateInstance index(ClaimStatus status);
    }

    @Inject
    ClaimState claimState;

    @GET
    @Produces(MediaType.TEXT_HTML)
    public TemplateInstance get() {
        ClaimStatus status = claimState.getCurrent();
        return Templates.index(status);
    }
}
