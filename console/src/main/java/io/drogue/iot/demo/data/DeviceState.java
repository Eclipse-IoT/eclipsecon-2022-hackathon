package io.drogue.iot.demo.data;

import com.fasterxml.jackson.annotation.JsonInclude;
import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class DeviceState {
    @JsonInclude(JsonInclude.Include.NON_NULL)
    private OnOffStatus button;
    @JsonInclude(JsonInclude.Include.NON_NULL)
    private BatteryStatus battery;
    @JsonInclude(JsonInclude.Include.NON_NULL)
    private SensorStatus sensor;

    public OnOffStatus getButton() {
        return button;
    }

    public void setButton(OnOffStatus button) {
        this.button = button;
    }

    public BatteryStatus getBattery() {
        return battery;
    }

    public void setBattery(BatteryStatus battery) {
        this.battery = battery;
    }

    public SensorStatus getSensor() {
        return sensor;
    }

    public void setSensor(SensorStatus sensor) {
        this.sensor = sensor;
    }

    @Override
    public String toString() {
        return "DeviceState{" +
                "button=" + button +
                ", battery=" + battery +
                ", sensor=" + sensor +
                '}';
    }
}
