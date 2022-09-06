package io.drogue.iot.hackathon.service;

import java.util.Optional;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.persistence.EntityManager;
import javax.transaction.Transactional;

import io.drogue.iot.hackathon.model.Claims;

@ApplicationScoped
public class DeviceClaimService {

    @Inject
    EntityManager em;

    @Transactional
    public Optional<DeviceClaim> getDeviceClaimFor(final String userId) {
        var cb = em.getCriteriaBuilder();
        var cr = cb.createQuery(Claims.class);
        var root = cr.from(Claims.class);
        cr.select(root).where(cb.equal(root.get("claimedBy"), userId));

        return em.createQuery(cr)
                .getResultStream()
                .findFirst()
                .map(claim -> {
                    var result = new DeviceClaim();
                    result.id = claim.getId();
                    result.deviceId = claim.getDeviceId();
                    return result;
                });
    }

    @Transactional
    public DeviceClaim claimDevice(final String claimId, final String userId, final boolean canCreate) throws AlreadyClaimedException {
        var claim = this.em.find(Claims.class, claimId);
        if (claim == null || claim.getClaimedBy() != null) {
            if (claim == null && canCreate) {
                claim = new Claims();
                claim.setId(claimId);
                // if we auto-create a claim, the deviceId is equal to the claimId
                claim.setDeviceId(claimId);
            } else {
                throw new AlreadyClaimedException(claimId);
            }
        }

        claim.setClaimedBy(userId);
        this.em.persist(claim);

        var result = new DeviceClaim();
        result.id = claimId;
        result.deviceId = claim.getDeviceId();
        return result;
    }

    @Transactional
    public boolean releaseDevice(final String userId) {

        var cb = em.getCriteriaBuilder();
        var cr = cb.createCriteriaDelete(Claims.class);
        var root = cr.from(Claims.class);
        cr.where(cb.equal(root.get("claimedBy"), userId));

        var updates = this.em.createQuery(cr).executeUpdate();

        return updates > 0;
    }

    @Transactional
    public boolean deleteClaim(final String claimId) {
        var cb = em.getCriteriaBuilder();
        var cr = cb.createCriteriaDelete(Claims.class);
        var root = cr.from(Claims.class);
        cr.where(cb.equal(root.get("id"), claimId));

        return this.em.createQuery(cr).executeUpdate() > 0;
    }
}
