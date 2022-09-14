package io.drogue.iot.hackathon.registry;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

import javax.enterprise.context.ApplicationScoped;
import javax.ws.rs.WebApplicationException;

import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.eclipse.microprofile.rest.client.inject.RestClient;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.fasterxml.jackson.databind.ObjectMapper;

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
        List<Device> devices = this.registryService.getDevices(this.applicationName, "role=gateway");
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
        metadata.setApplication(this.applicationName);
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
        this.registryService.createDevice(this.applicationName, dev);
    }

    public void createSimulatorDevice(String device, String password) {
        var dev = new Device();
        var metadata = new Metadata();
        metadata.setName(device);
        metadata.setApplication(this.applicationName);
        dev.setMetadata(metadata);

        var spec = new DeviceSpec();
        dev.setSpec(spec);

        var credentials = new CredentialsSpec();
        credentials.setCredentials(List.of(new Password(password)));
        spec.setCredentials(credentials);

        this.registryService.createDevice(this.applicationName, dev);
    }

    public void deleteDevice(String deviceId) {
        this.registryService.deleteDevice(this.applicationName, deviceId);
    }

    public Optional<Device> getDevice(String device) {
        try {
            return Optional.ofNullable(this.registryService.getDevice(this.applicationName, device));
        } catch (WebApplicationException e) {
            if (e.getResponse().getStatus() == 404) {
                return Optional.empty();
            }
            throw e;
        }
    }
}
