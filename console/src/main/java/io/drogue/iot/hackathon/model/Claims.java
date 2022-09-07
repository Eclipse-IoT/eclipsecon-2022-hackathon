package io.drogue.iot.hackathon.model;

import javax.persistence.Column;
import javax.persistence.Entity;
import javax.persistence.Id;
import javax.persistence.Index;
import javax.persistence.Table;

@Entity
@Table(
        indexes = {
                @Index(columnList = ("claimedBy"), unique = true),
                @Index(columnList = ("deviceId"), unique = true),
        }
)
public class Claims {
    /**
     * The claim ID/name, as printed on the box.
     */
    @Id
    @Column(nullable = false, unique = true)
    private String id;

    /**
     * The device ID, as registered in Drogue IoT.
     */
    @Column(nullable = false, unique = true)
    private String deviceId;

    /**
     * The name of the user claiming the device.
     * <p>
     * May be null.
     */
    private String claimedBy;

    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public String getDeviceId() {
        return deviceId;
    }

    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public void setClaimedBy(String claimedBy) {
        this.claimedBy = claimedBy;
    }

    public String getClaimedBy() {
        return claimedBy;
    }
}
