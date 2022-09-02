package io.drogue.iot.hackathon.integration.registry;

import io.quarkus.runtime.Startup;
import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.eclipse.microprofile.rest.client.inject.RestClient;

import javax.enterprise.context.ApplicationScoped;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

@Startup
@ApplicationScoped
public class Registry {

    @ConfigProperty(name = "drogue.application.name")
    String applicationName;

    @RestClient
    RegistryService registryService;

    public void createDevice(String device, String[] aliases) {
        // List gateways
        List<String> gateways = new ArrayList<>();
        List<Device> devices = registryService.getDevices(applicationName, "role=gateway");
        if (devices != null) {
            for (Device gateway : devices) {
                gateways.add(gateway.getMetadata().getName());
            }
        }

        // Create device struct
        Device dev = new Device();
        Metadata metadata = new Metadata();
        metadata.setName(device);
        dev.setMetadata(metadata);

        DeviceSpec spec = new DeviceSpec();
        DeviceAliases deviceAliases = new DeviceAliases();
        deviceAliases.setAliases(Arrays.asList(aliases));
        spec.setAlias(deviceAliases);

        GatewaySelector selector = new GatewaySelector();
        selector.setMatchNames(gateways);
        spec.setSelector(selector);

        dev.setSpec(spec);

        // Post device
        registryService.createDevice(applicationName, dev);
    }
}
