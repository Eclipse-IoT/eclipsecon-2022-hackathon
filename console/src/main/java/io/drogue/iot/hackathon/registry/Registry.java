package io.drogue.iot.hackathon.registry;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import javax.enterprise.context.ApplicationScoped;
import javax.ws.rs.core.Response;

import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.eclipse.microprofile.rest.client.ext.ResponseExceptionMapper;
import org.eclipse.microprofile.rest.client.inject.RestClient;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.fasterxml.jackson.databind.ObjectMapper;

import io.quarkus.runtime.Startup;

@Startup
@ApplicationScoped
public class Registry {
    private static final Logger LOG = LoggerFactory.getLogger(Registry.class);

    @ConfigProperty(name = "drogue.application.name")
    String applicationName;

    @RestClient
    RegistryService registryService;

    public void createDevice(String device, String uuid) {
        // List gateways
        List<String> gateways = new ArrayList<>();
        List<Device> devices = registryService.getDevices(applicationName, "role=gateway");
        if (devices != null) {
            for (Device gateway : devices) {
                gateways.add(gateway.getMetadata().getName());
            }
        }

        LOG.info("Using gateways {}", gateways);

        // Create device struct
        Device dev = new Device();
        Metadata metadata = new Metadata();
        metadata.setName(device);
        metadata.setApplication(applicationName);
        dev.setMetadata(metadata);

        DeviceSpec spec = new DeviceSpec();
        GatewaySelector selector = new GatewaySelector();
        selector.setMatchNames(gateways);
        spec.setGatewaySelector(selector);
        BtMeshSpec mesh = new BtMeshSpec();
        mesh.setDevice(uuid);
        spec.setBtmesh(mesh);

        dev.setSpec(spec);

        try {
            LOG.info("Creating device: {}", new ObjectMapper().writeValueAsString(dev));
        } catch (Exception e) {
            // Ignored
        }

        // Post device
        registryService.createDevice(applicationName, dev);
    }

    public void deleteDevice(String deviceId) {
        registryService.deleteDevice(applicationName, deviceId);
    }
}
