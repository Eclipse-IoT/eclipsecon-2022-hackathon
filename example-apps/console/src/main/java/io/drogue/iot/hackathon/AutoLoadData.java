package io.drogue.iot.hackathon;

import java.io.IOException;

import javax.enterprise.context.ApplicationScoped;
import javax.enterprise.event.Observes;
import javax.inject.Inject;
import javax.transaction.Transactional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.databind.ObjectMapper;

import io.drogue.iot.hackathon.service.DeviceClaimService;
import io.quarkus.runtime.StartupEvent;
import io.quarkus.runtime.annotations.RegisterForReflection;

@ApplicationScoped
public class AutoLoadData {

    private final static Logger logger = LoggerFactory.getLogger(AutoLoadData.class);

    @RegisterForReflection
    @JsonIgnoreProperties(ignoreUnknown=true)
    static class IdMapEntry {
        public String id;

        public String address;

        public String devkey;
    }

    @Inject
    DeviceClaimService service;

    public void onStartup(@Observes StartupEvent ev) throws IOException {
        loadMappings();
    }

    @Transactional
    void loadMappings() throws IOException {
        var resource = Thread.currentThread().getContextClassLoader().getResourceAsStream("META-INF/resources/idmap.json");
        IdMapEntry[] items = new ObjectMapper().readValue(resource, IdMapEntry[].class);
        for (IdMapEntry item : items) {
            this.service.createClaim(item.id, item.address);
        }
        logger.info("Auto-loaded {} mappings", items.length);
    }

}
