package io.drogue.iot.demo.ui;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class DisplaySettings {
    public Integer brightness;
    public Boolean enabled;
}
