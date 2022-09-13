package io.drogue.iot.hackathon.registrty;

import java.util.List;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import io.drogue.iot.hackathon.registry.Credentials;
import io.drogue.iot.hackathon.registry.CredentialsSpec;
import io.drogue.iot.hackathon.registry.Password;
import io.vertx.core.json.Json;

public class CredentialsTest {

    @Test
    void testDeserializeEmpty() {
        var credentials = Json.decodeValue("{\"credentials\": []}", CredentialsSpec.class);
        Assertions.assertTrue(credentials.getCredentials().isEmpty());
    }

    @Test
    void testDeserializePlain() {
        var credentials = Json.decodeValue("{\"credentials\": [{\"pass\": \"foo\"}]}", CredentialsSpec.class);
        Assertions.assertArrayEquals(new Credentials[] {
                new Password("foo"),
        }, credentials.getCredentials().toArray(new Credentials[0]));
    }

    @Test
    void testDeserializeHashed() {
        var credentials = Json.decodeValue("{\"credentials\": [{\"pass\": {\"sha512\": \"$6$9wbS5ikTE76frCNT$iupGAHSz.CtA1FYu1AHZ6piytp.CkbxSpl43V/3k7JgMoNljWlEiPt0b0y8LadRfriB/w5M1vwTfeTGqHvhzr.\"}}]}", CredentialsSpec.class);
        Assertions.assertArrayEquals(new Credentials[] {
                new Password("$6$9wbS5ikTE76frCNT$iupGAHSz.CtA1FYu1AHZ6piytp.CkbxSpl43V/3k7JgMoNljWlEiPt0b0y8LadRfriB/w5M1vwTfeTGqHvhzr.", "sha512"),
        }, credentials.getCredentials().toArray(new Credentials[0]));
    }

    @Test
    void testSerialize() {
        var credentials = new CredentialsSpec();
        credentials.setCredentials(List.of(
                new Password("foo"),
                new Password("bar", "hash")
        ));
        Assertions.assertEquals("{\"credentials\":[{\"pass\":\"foo\"},{\"pass\":{\"hash\":\"bar\"}}]}", Json.encode(credentials));

    }

}
