package io.drogue.iot.hackathon.integration;

import static io.cloudevents.core.CloudEventUtils.mapData;

import java.time.Instant;
import java.time.OffsetDateTime;
import java.util.Optional;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;

import org.eclipse.microprofile.reactive.messaging.Incoming;
import org.eclipse.microprofile.reactive.messaging.Message;
import org.eclipse.microprofile.reactive.messaging.OnOverflow;
import org.eclipse.microprofile.reactive.messaging.Outgoing;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.fasterxml.jackson.databind.ObjectMapper;

import io.cloudevents.core.provider.EventFormatProvider;
import io.cloudevents.jackson.JsonFormat;
import io.cloudevents.jackson.PojoCloudEventDataMapper;
import io.drogue.iot.hackathon.data.DeviceEvent;
import io.drogue.iot.hackathon.data.DevicePayload;
import io.quarkus.runtime.Startup;
import io.smallrye.reactive.messaging.annotations.Broadcast;

/**
 * Receive events from the Drogue IoT MQTT integration.
 */
@Startup
@ApplicationScoped
public class Receiver {

    private static final Logger LOG = LoggerFactory.getLogger(Receiver.class);

    @Inject
    ObjectMapper objectMapper;

    /**
     * Receive an event, parse into a Cloud Event, and extract the TTN uplink information.
     *
     * @param rawMessage The raw MQTT message.
     * @return The processed {@link DeviceEvent}, or {@code null} if the event couldn't be processed.
     */
    @Incoming("telemetry")
    @OnOverflow(value = OnOverflow.Strategy.DROP)
    @Outgoing("event-stream")
    @Broadcast
    public DeviceEvent process(Message<byte[]> rawMessage) {

        // we always ack, as we don't care about errors in this demo

        rawMessage.ack();

        // start processing

        var format = EventFormatProvider
                .getInstance()
                .resolveFormat(JsonFormat.CONTENT_TYPE);

        if (format == null) {
            // failed to get decoder
            LOG.warn("Failed to get CloudEvents decoder");
            return null;
        }

        try {

            var event = format.deserialize(rawMessage.getPayload());

            if (!"io.drogue.event.v1".equals(event.getType())) {
                // we are only interested in telemetry events
                return null;
            }

            if (!"sensor".equals(event.getSubject())) {
                // we are only interested in the "sensor" channel
                return null;
            }

            var payload = mapData(
                    event,
                    PojoCloudEventDataMapper.from(this.objectMapper, DevicePayload.class)
            );
            if (payload == null) {
                // ignore if we are missing payload
                return null;
            }

            var deviceId = Optional.ofNullable(event.getExtension("device"))
                    .map(Object::toString)
                    .orElse(null);
            if (deviceId == null) {
                // ignore if we are missing information
                return null;
            }

            // create device event

            var device = new DeviceEvent();
            device.setDeviceId(deviceId);
            device.setTimestamp(Optional.ofNullable(event.getTime())
                    .map(OffsetDateTime::toInstant)
                    .orElseGet(Instant::now));
            device.setPayload(payload.getValue());

            // done

            return device;

        } catch (Exception e) {
            LOG.debug("Error decoding: {}", e.getMessage());
            return null;
        }
    }

}
