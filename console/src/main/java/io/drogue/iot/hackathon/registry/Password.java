package io.drogue.iot.hackathon.registry;

import java.util.Objects;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class Password implements Credentials {
    private final String value;

    private final String algorithm;

    public Password(String plain) {
        this.value = plain;
        this.algorithm = null;
    }

    public Password(String value, String algorithm) {
        this.value = value;
        this.algorithm = algorithm;
    }

    public String getAlgorithm() {
        return this.algorithm;
    }

    public String getValue() {
        return this.value;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        Password password = (Password) o;
        return Objects.equals(this.value, password.value) && Objects.equals(this.algorithm, password.algorithm);
    }

    @Override
    public int hashCode() {
        return Objects.hash(this.value, this.algorithm);
    }

}
