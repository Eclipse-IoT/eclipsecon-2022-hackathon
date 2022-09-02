package io.drogue.iot.hackathon.ui;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class DisplaySettings {
    public String device;

    public Integer brightness;
    public Boolean enabled;
}
