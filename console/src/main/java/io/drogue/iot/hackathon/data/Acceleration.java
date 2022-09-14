package io.drogue.iot.hackathon.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class Acceleration {
    public Long x;

    public Long y;

    public Long z;
}
