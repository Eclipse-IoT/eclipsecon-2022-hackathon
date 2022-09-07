package io.drogue.iot.hackathon.data;

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

    public DeviceState merge(DeviceState update) {
        var outcome = this.clone();

        if (update.battery != null) {
            outcome.battery = update.battery;
        }
        if (update.button != null) {
            outcome.button = update.button;
        }
        if (update.sensor != null) {
            outcome.sensor = update.sensor;
        }

        return outcome;
    }

    @SuppressWarnings("MethodDoesntCallSuperMethod")
    @Override
    public DeviceState clone() {
        var outcome = new DeviceState();
        outcome.battery = this.battery;
        outcome.button = this.button;
        outcome.sensor = this.sensor;
        return outcome;
    }
}
