package io.drogue.iot.hackathon.service;

import com.fasterxml.jackson.databind.ObjectMapper;
import io.quarkus.runtime.annotations.RegisterForReflection;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.enterprise.context.ApplicationScoped;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

@ApplicationScoped
@RegisterForReflection
public class IdMap {
    private static final Logger LOG = LoggerFactory.getLogger(IdMap.class);

    private static final Map<String, String> map;

    static {
        try {
            Map<String, String> mapping = new HashMap<>();
            var resource = Thread.currentThread().getContextClassLoader().getResourceAsStream("META-INF/resources/idmap.json");
            IdMapEntry[] items = new ObjectMapper().readValue(resource, IdMapEntry[].class);
            for (IdMapEntry item : items) {
                mapping.put(item.id, item.uuid);
            }
            LOG.info("Loaded {} entries in id map", items.length);
            map = mapping;
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public String get(String id) {
        return map.get(id);
    }
}
