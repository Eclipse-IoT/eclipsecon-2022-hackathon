package io.drogue.iot.hackathon.service;

import com.google.common.base.MoreObjects;

public class DeviceClaim {
    public String id;

    public String deviceId;

    @Override
    public String toString() {
        return MoreObjects.toStringHelper(this)
                .add("id", id)
                .add("deviceId", deviceId)
                .toString();
    }
}
