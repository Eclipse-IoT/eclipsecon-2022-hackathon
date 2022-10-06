package io.drogue.iot.hackathon.endpoints.render;

import java.util.Optional;
import java.util.function.Function;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class Cell {
    private static final Logger logger = LoggerFactory.getLogger(Cell.class);

    private final Object value;

    private final String renderedValue;

    private Cell(final Object value, final String renderedValue) {
        this.value = value;
        this.renderedValue = renderedValue;
    }

    @Override
    public String toString() {
        return this.renderedValue;
    }

    @SuppressWarnings("rawtypes")
    public Comparable comparableValue() {
        var value = this.value;

        if (value instanceof Optional) {
            value = ((Optional<?>) value).orElse(null);
        }

        if (value == null) {
            return null;
        }

        if (!(value instanceof Comparable)) {
            value = value.toString();
        }

        return (Comparable<?>) value;
    }

    public static <T> Cell cell(final Optional<T> value) {
        return cell(value, Object::toString, "");
    }

    public static <T> Cell cell(final Optional<T> value, final Function<T, String> render) {
        return cell(value, render, "");
    }

    public static <T> Cell cell(final Optional<T> value, final Function<T, String> render, final String other) {
        return new Cell(value, value.map(render).orElse(other));
    }

}
