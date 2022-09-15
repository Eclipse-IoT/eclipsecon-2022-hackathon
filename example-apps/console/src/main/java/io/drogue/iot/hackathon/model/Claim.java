package io.drogue.iot.hackathon.model;

import javax.persistence.Column;
import javax.persistence.Entity;
import javax.persistence.Id;
import javax.persistence.Index;
import javax.persistence.Table;

@Entity
@Table(
        name = "claims",
        indexes = {
                @Index(columnList = ("claimed_by"), unique = true),
        }
)
public class Claim {
    /**
     * The claim ID/name, as printed on the box.
     * <p>
     * Also used as "device name" in the device registry.
     */
    @Id
    @Column(nullable = false, unique = true)
    private String id;

    /**
     * The ID used for provisioning a device.
     */
    @Column(nullable = false, unique = true, name = "provisioning_id")
    private String provisioningId;

    /**
     * The name of the user claiming the device.
     * <p>
     * May be null.
     */
    @Column(name = "claimed_by")
    private String claimedBy;

    public String getId() {
        return this.id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public String getProvisioningId() {
        return this.provisioningId;
    }

    public void setProvisioningId(String provisioningId) {
        this.provisioningId = provisioningId;
    }

    public void setClaimedBy(String claimedBy) {
        this.claimedBy = claimedBy;
    }

    public String getClaimedBy() {
        return this.claimedBy;
    }
}
