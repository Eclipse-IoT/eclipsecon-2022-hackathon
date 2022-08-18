FROM fedora-minimal:36

LABEL org.opencontainers.image.source="https://github.com/eclipse-iot/eclipsecon-2022-hackathon"

COPY target/release/eclipsecon-model-converter /

ENTRYPOINT [ "/eclipsecon-model-converter" ]
