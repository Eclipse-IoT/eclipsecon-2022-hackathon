package io.drogue.iot.demo.integration;

import javax.enterprise.context.ApplicationScoped;

import io.drogue.iot.demo.DeviceCommand;
import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.eclipse.microprofile.reactive.messaging.Incoming;
import org.eclipse.microprofile.reactive.messaging.Message;
import org.eclipse.microprofile.reactive.messaging.Outgoing;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.drogue.iot.demo.data.CommandPayload;
import io.netty.handler.codec.mqtt.MqttQoS;
import io.quarkus.runtime.Startup;
import io.smallrye.reactive.messaging.mqtt.MqttMessage;

/**
 * Send commands to the Drogue IoT MQTT integration.
 */
@Startup
@ApplicationScoped
public class Sender {

    private static final Logger LOG = LoggerFactory.getLogger(Sender.class);

    @ConfigProperty(name = "drogue.application.name")
    String applicationName;

    @Incoming("device-commands")
    @Outgoing("commands")
    public Message<byte[]> commands(DeviceCommand command) {
        LOG.info("Request to send device command: {}", command);

        var topic = "command/" + this.applicationName + "/" + command.getDeviceId() + "/port:1";

        LOG.info("Sending to topic: {}", topic);
        LOG.info("Sending payload: {}", command.getPayload());

        return MqttMessage.of(topic, command.getPayload(), MqttQoS.AT_LEAST_ONCE);
    }

}
