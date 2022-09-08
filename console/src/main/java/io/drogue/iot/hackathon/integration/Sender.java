package io.drogue.iot.hackathon.integration;

import javax.enterprise.context.ApplicationScoped;

import com.fasterxml.jackson.databind.ObjectMapper;
import io.drogue.iot.hackathon.commands.DeviceCommand;
import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.eclipse.microprofile.reactive.messaging.Incoming;
import org.eclipse.microprofile.reactive.messaging.Message;
import org.eclipse.microprofile.reactive.messaging.Outgoing;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

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
    public Message<byte[]> deviceCommands(DeviceCommand command) {
        LOG.info("Request to send device command: {}", command);

        var topic = "command/" + this.applicationName + "/" + command.getDeviceId() + "/sensor";

        LOG.info("Sending to topic: {}", topic);

        return MqttMessage.of(topic, command.getPayload(), MqttQoS.AT_LEAST_ONCE);
    }

    @Incoming("provisioner-commands")
    @Outgoing("commands")
    public Message<byte[]> gatewayCommands(ProvisioningCommand command) {
        LOG.info("Request to send provisioner command: {}", command);
        var topic = "command/" + this.applicationName + "/provisioner/provision";

        LOG.info("Sending to topic: {}", topic);
        var m = new ObjectMapper();
        try {
            byte[] payload = m.writeValueAsBytes(command);
            return MqttMessage.of(topic, payload, MqttQoS.AT_LEAST_ONCE);
        } catch (Exception e) {
            e.printStackTrace();
            return null;
        }
    }
}
