FROM ghcr.io/drogue-iot/builder:0.1.20 as builder

RUN mkdir /build
ADD . /build
WORKDIR /build/btmesh-operator

RUN cargo build --release

FROM registry.access.redhat.com/ubi8-minimal

LABEL org.opencontainers.image.source="https://github.com/eclipse-iot/eclipsecon-2022-hackathon"

COPY --from=builder /build/target/release/btmesh-operator /
COPY --from=builder /build/btmesh-operator/scripts/start.sh /

ENTRYPOINT [ "/start.sh" ]
