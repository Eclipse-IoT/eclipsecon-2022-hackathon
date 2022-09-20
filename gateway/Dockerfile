FROM docker.io/library/fedora:36 as builder

RUN dnf install -y dbus-devel openssl-devel cmake pkgconf-pkg-config gcc gcc-c++ make

ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/rust

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.63.0

ENV PATH "$PATH:$CARGO_HOME/bin"

RUN mkdir /build
ADD . /build
WORKDIR /build/gateway

RUN cargo build --release

FROM registry.access.redhat.com/ubi9-minimal

LABEL org.opencontainers.image.source="https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon"

RUN microdnf install -y dbus-libs

COPY --from=builder /build/target/release/eclipsecon-gateway /

ENTRYPOINT [ "/eclipsecon-gateway" ]
