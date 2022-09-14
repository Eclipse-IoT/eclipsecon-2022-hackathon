package io.drogue.iot.hackathon.data;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class SensorPayload {
    private Long temperature;

    private Acceleration acceleration;

    private Long noise;

    public Long getTemperature() {
        return this.temperature;
    }

    public void setTemperature(Long temperature) {
        this.temperature = temperature;
    }

    public Long getNoise() {
        return this.noise;
    }

    public void setNoise(Long noise) {
        this.noise = noise;
    }

    public Acceleration getAcceleration() {
        return this.acceleration;
    }

    public void setAcceleration(Acceleration acceleration) {
        this.acceleration = acceleration;
    }

    @Override
    public String toString() {
        return "SensorPayload{" +
                "temperature=" + this.temperature +
                ", acceleration=" + this.acceleration +
                ", noise=" + this.noise +
                '}';
    }
}
