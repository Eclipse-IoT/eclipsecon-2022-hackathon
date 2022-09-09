package io.drogue.iot.hackathon;

import java.util.HashMap;
import java.util.Map;

import javax.enterprise.context.ApplicationScoped;

import com.google.common.base.MoreObjects;

import io.drogue.iot.hackathon.model.BasicFeature;

@ApplicationScoped
public class StateHolder {
    private volatile Map<String, Map<String, BasicFeature>> state = new HashMap<>();

    public Map<String, Map<String, BasicFeature>> getState() {
        return this.state;
    }

    public void setState(Map<String, Map<String, BasicFeature>> state) {
        this.state = state;
    }

    @Override
    public String toString() {
        return MoreObjects.toStringHelper(this)
                .add("state", this.state)
                .toString();
    }
}
