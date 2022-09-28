FROM ghcr.io/drogue-iot/builder:0.2.3 as builder

RUN mkdir /build
ADD . /build
WORKDIR /build/infra/model-converter

RUN cargo build --release

FROM registry.access.redhat.com/ubi9-minimal

LABEL org.opencontainers.image.source="https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon"

COPY --from=builder /build/infra/target/release/eclipsecon-model-converter /

ENTRYPOINT [ "/eclipsecon-model-converter" ]
