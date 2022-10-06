package io.drogue.iot.hackathon.endpoints.render;

import java.util.Arrays;
import java.util.Comparator;
import java.util.LinkedList;
import java.util.List;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class Table {
    private static final Logger logger = LoggerFactory.getLogger(Table.class);

    private final List<String> header;

    private final List<List<Cell>> rows = new LinkedList<>();

    private int sortedBy = -1;

    private Direction direction;

    public Table(final String... header) {
        this.header = List.of(header);
    }

    public void addRow(final Cell... cells) {
        if (cells.length != this.header.size()) {
            throw new IllegalArgumentException(String.format(
                    "Expect %s values based on the provided headers, got only %s",
                    this.header.size(), cells.length
            ));
        }

        // we cannot use List.of as we might have null values
        this.rows.add(Arrays.asList(cells));
    }

    public List<List<Cell>> getRows() {
        return this.rows;
    }

    public List<String> getHeader() {
        return this.header;
    }

    public int getSortedBy() {
        return this.sortedBy;
    }

    public Direction getDirection() {
        return this.direction;
    }

    public String directionForColumn(final int column) {
        if (this.sortedBy != column) {
            return "none";
        } else {
            return this.direction.name().toLowerCase();
        }
    }

    public String columnSelected(final int column, final String value) {
        if (this.sortedBy == column) {
            return value;
        } else {
            return "";
        }
    }

    @SuppressWarnings("unchecked")
    public void sortBy(final int column, Direction direction) {

        if (direction == null) {
            direction = Direction.ASCENDING;
        }

        if (this.sortedBy == column && this.direction == direction) {
            // no need to re-sort
            return;
        }

        if (column < 0 || column >= this.header.size()) {
            // nothing do to here
            this.direction = null;
            return;
        }

        this.sortedBy = column;
        this.direction = direction;

        @SuppressWarnings("rawtypes")
        Comparator comparator = Comparator.naturalOrder();
        if (direction == Direction.DESCENDING) {
            comparator = comparator.reversed();
        }

        // make final
        final var comp = Comparator.nullsLast(comparator);

        this.rows.sort((o1, o2) -> {
            final var v1 = o1.get(column).comparableValue();
            final var v2 = o2.get(column).comparableValue();

            return comp.compare(v1, v2);
        });
    }
}
