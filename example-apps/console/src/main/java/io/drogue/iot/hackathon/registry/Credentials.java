package io.drogue.iot.hackathon.registry;

import java.io.IOException;

import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;
import com.fasterxml.jackson.databind.ser.std.StdSerializer;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
@JsonDeserialize(using = Credentials.Deserializer.class)
@JsonSerialize(using = Credentials.Serializer.class)
public interface Credentials {
    class Deserializer extends StdDeserializer<Credentials> {

        public Deserializer() {
            super(Credentials.class);
        }

        @Override
        public Credentials deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
            var node = p.getCodec().readTree(p);
            if (node instanceof TextNode) {
                return new Password(((TextNode) node).asText());
            }
            if (node instanceof ObjectNode) {
                var pass = node.get("pass");
                if (pass instanceof TextNode) {
                    return new Password(((TextNode) pass).asText());
                }
                if (pass instanceof ObjectNode) {
                    var entry = ((ObjectNode) pass).fields().next();
                    if (entry != null) {
                        return new Password(entry.getValue().asText(), entry.getKey());
                    }
                }
            }

            return null;
        }
    }

    class Serializer extends StdSerializer<Credentials> {

        public Serializer() {
            super(Credentials.class);
        }

        @Override
        public void serialize(Credentials value, JsonGenerator gen, SerializerProvider provider) throws IOException {
            if (value instanceof Password) {
                gen.writeStartObject();
                gen.writeFieldName("pass");
                var pwd = (Password) value;
                if (pwd.getAlgorithm() != null) {
                    gen.writeStartObject();
                    gen.writeStringField(pwd.getAlgorithm(), pwd.getValue());
                    gen.writeEndObject();
                } else {
                    gen.writeString(pwd.getValue());
                }
                gen.writeEndObject();
            }
        }
    }
}
