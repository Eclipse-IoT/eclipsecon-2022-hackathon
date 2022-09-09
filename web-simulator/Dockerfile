FROM --platform=$BUILDPLATFORM ghcr.io/drogue-iot/builder:0.2.3 as builder

# we need to pull in the full source directory, as we reference local crates
RUN mkdir -p /build
ADD . /build
WORKDIR /build/web-simulator

RUN trunk build --release

FROM ghcr.io/drogue-iot/frontend-base:0.2.0

LABEL org.opencontainers.image.source="https://github.com/eclipse-iot/eclipsecon-2022-hackathon"

RUN mkdir /public

COPY --from=builder /build/web-simulator/nginx.conf /etc/nginx/nginx.conf

COPY --from=builder /build/web-simulator/nginx.sh /nginx.sh
RUN chmod a+x /nginx.sh

CMD ["/nginx.sh"]

COPY --from=builder /build/web-simulator/dist/ /public/

EXPOSE 8080
