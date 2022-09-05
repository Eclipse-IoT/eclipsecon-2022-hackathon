package io.drogue.iot.hackathon.data;

public class SensorPayload {
    private Long temperature;
    private Acceleration acceleration;

    public Long getTemperature() {
        return temperature;
    }

    public void setTemperature(Long temperature) {
        this.temperature = temperature;
    }

    @Override
    public String toString() {
        return "SensorPayload{" +
                "temperature=" + temperature +
                '}';
    }
}
