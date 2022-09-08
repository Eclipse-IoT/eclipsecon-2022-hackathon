package io.drogue.iot.hackathon.integration;

import static io.cloudevents.core.CloudEventUtils.mapData;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;

import io.drogue.iot.hackathon.data.DevicePayload;
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
        try {

            var event = format.deserialize(rawMessage.getPayload());

            if ("sensor".equals(event.getSubject())) {
                var payload = mapData(
                        event,
                        PojoCloudEventDataMapper.from(this.objectMapper, DevicePayload.class)
                );

                // create device event

                var device = new DeviceEvent();
                device.setDeviceId(event.getExtension("device").toString());
                device.setTimestamp(event.getTime().toInstant());
                device.setPayload(payload.getValue());

                // done

                return device;
            }
            return null;
        } catch (Exception e) {
            LOG.debug("Error decoding: {}", e.getMessage());
            return null;
        }
    }

}
