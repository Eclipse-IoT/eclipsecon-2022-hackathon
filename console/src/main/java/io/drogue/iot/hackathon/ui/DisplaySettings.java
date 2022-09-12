package io.drogue.iot.hackathon.ui;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class DisplaySettings {
    public String device;
    public Long address;

    public Integer brightness;

    public boolean enabled;
}
