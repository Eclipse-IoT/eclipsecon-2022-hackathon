package io.drogue.iot.hackathon.ui;

import java.util.Optional;

import javax.inject.Inject;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import io.drogue.iot.hackathon.service.DeviceClaim;
import io.drogue.iot.hackathon.service.DeviceClaimService;
import io.quarkus.qute.CheckedTemplate;
import io.quarkus.qute.TemplateInstance;
import io.smallrye.common.annotation.Blocking;

@Path("/")
public class Index {

    @CheckedTemplate
    public static class Templates {
        public static native TemplateInstance index(Optional<DeviceClaim> status);
    }

    @Inject
    DeviceClaimService deviceClaim;

    @GET
    @Produces(MediaType.TEXT_HTML)
    @Blocking
    public TemplateInstance get() {
        var status = this.deviceClaim.getDeviceClaimFor(null);
        return Templates.index(status);
    }
}
