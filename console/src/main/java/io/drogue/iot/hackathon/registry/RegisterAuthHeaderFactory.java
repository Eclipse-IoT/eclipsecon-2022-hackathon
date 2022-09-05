package io.drogue.iot.hackathon.registry;

import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.eclipse.microprofile.rest.client.ext.ClientHeadersFactory;

import javax.enterprise.context.ApplicationScoped;
import javax.ws.rs.core.MultivaluedMap;
import java.util.Base64;

@ApplicationScoped
public class RegisterAuthHeaderFactory implements ClientHeadersFactory {

    @ConfigProperty(name = "drogue.api.user") String user;
    @ConfigProperty(name = "drogue.api.key") String key;

    @Override
    public MultivaluedMap<String, String> update(MultivaluedMap<String, String> incomingHeaders, MultivaluedMap<String, String> clientOutgoingHeaders) {
        var auth = "Basic " + Base64.getEncoder().encodeToString((user + ":" + key).getBytes());
        clientOutgoingHeaders.add("Authorization", auth);
        return clientOutgoingHeaders;
    }
}
