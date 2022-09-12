package io.drogue.iot.hackathon.data;

public class SensorPayload {
    private Long temperature;
    private Acceleration acceleration;
    private Long noise;

    public Long getTemperature() {
        return temperature;
    }

    public void setTemperature(Long temperature) {
        this.temperature = temperature;
    }

    public Long getNoise() {
        return noise;
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
                "temperature=" + temperature +
                ", acceleration=" + acceleration +
                ", noise=" + noise +
                '}';
    }
}
