FROM ghcr.io/drogue-iot/builder:0.1.20 as builder

RUN mkdir /build
ADD . /build
WORKDIR /build/model-converter

RUN cargo build --release

FROM registry.access.redhat.com/ubi8-minimal

LABEL org.opencontainers.image.source="https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon"

RUN mkdir /public

COPY --from=builder /build/target/release/eclipsecon-model-converter /

ENTRYPOINT [ "/eclipsecon-model-converter" ]
