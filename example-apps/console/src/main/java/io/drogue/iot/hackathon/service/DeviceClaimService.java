package io.drogue.iot.hackathon.service;

import java.util.Optional;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.persistence.EntityManager;
import javax.transaction.Transactional;

import io.drogue.iot.hackathon.model.Claim;

@ApplicationScoped
public class DeviceClaimService {

    @Inject
    EntityManager em;

    @Transactional
    public Optional<DeviceClaim> getDeviceClaimFor(final String userId) {
        var cb = this.em.getCriteriaBuilder();
        var cr = cb.createQuery(Claim.class);
        var root = cr.from(Claim.class);
        cr.select(root)
                .where(cb
                        .equal(root.get("claimedBy"), userId
                        ));

        return this.em.createQuery(cr)
                .getResultStream()
                .findFirst()
                .map(claim -> new DeviceClaim(claim.getId(), claim.getProvisioningId()));
    }

    @Transactional
    public DeviceClaim claimDevice(final String claimId, final String userId, final boolean canCreate) throws AlreadyClaimedException {
        var claim = this.em.find(Claim.class, claimId);
        if (claim == null || claim.getClaimedBy() != null) {
            if (claim == null && canCreate) {
                claim = new Claim();
                claim.setId(claimId);
                // if we auto-create a claim, the provisioningId is equal to the claimId
                claim.setProvisioningId(claimId);
            } else {
                throw new AlreadyClaimedException(claimId);
            }
        }

        claim.setClaimedBy(userId);
        this.em.persist(claim);

        return new DeviceClaim(claimId, claim.getProvisioningId());
    }

    @Transactional
    public boolean releaseDevice(final String userId) {
        var cb = this.em.getCriteriaBuilder();
        var cr = cb.createCriteriaUpdate(Claim.class);
        var root = cr.from(Claim.class);
        cr
                .where(cb.equal(
                        root.get("claimedBy"), userId)
                )
                .set("claimedBy", null);

        var updates = this.em.createQuery(cr).executeUpdate();

        return updates > 0;
    }

    @Transactional
    public boolean deleteClaim(final String claimId) {
        var cb = this.em.getCriteriaBuilder();
        var cr = cb.createCriteriaDelete(Claim.class);
        var root = cr.from(Claim.class);
        cr.where(cb.equal(root.get("id"), claimId));

        return this.em.createQuery(cr).executeUpdate() > 0;
    }

    /**
     * Create a new claim, ignoring existing entries.
     *
     * @param id The claim id.
     * @param provisioningId The provisioning id.
     */
    public void createClaim(String id, String provisioningId) {
        var claim = new Claim();
        claim.setId(id);
        claim.setProvisioningId(provisioningId);
        this.em.merge(claim);
    }
}
