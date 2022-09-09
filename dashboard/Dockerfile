FROM registry.access.redhat.com/ubi9/ubi:latest AS builder

RUN dnf -y install java maven rsync
RUN mkdir /usr/src/build
COPY . /usr/src/build
WORKDIR /usr/src/build
RUN mvn -B package

FROM registry.access.redhat.com/ubi9/ubi-minimal:latest

LABEL org.opencontainers.image.source="https://github.com/Eclipse-IoT/eclipsecon-2022-hackathon"

ARG JAVA_PACKAGE=java-11-openjdk-headless
ARG RUN_JAVA_VERSION=1.3.8
ENV LANG='en_US.UTF-8' LANGUAGE='en_US:en'
# Install java and the run-java script
# Also set up permissions for user `1001`
RUN microdnf -y install ca-certificates ${JAVA_PACKAGE} \
    && microdnf -y update \
    && microdnf -y clean all \
    && mkdir /deployments \
    && chown 1001 /deployments \
    && chmod "g+rwX" /deployments \
    && chown 1001:root /deployments \
    && curl https://repo1.maven.org/maven2/io/fabric8/run-java-sh/${RUN_JAVA_VERSION}/run-java-sh-${RUN_JAVA_VERSION}-sh.sh -o /deployments/run-java.sh \
    && chown 1001 /deployments/run-java.sh \
    && chmod 540 /deployments/run-java.sh \
    && echo "securerandom.source=file:/dev/urandom" >> /etc/alternatives/jre/lib/security/java.security

# Configure the JAVA_OPTIONS, you can add -XshowSettings:vm to also display the heap size.
ENV JAVA_OPTIONS="-Dquarkus.http.host=0.0.0.0 -Djava.util.logging.manager=org.jboss.logmanager.LogManager"
# We make four distinct layers so if there are application changes the library layers can be re-used
COPY --from=builder --chown=1001 /usr/src/build/target/quarkus-app/lib/ /deployments/lib/
COPY --from=builder --chown=1001 /usr/src/build/target/quarkus-app/*.jar /deployments/
COPY --from=builder --chown=1001 /usr/src/build/target/quarkus-app/app/ /deployments/app/
COPY --from=builder --chown=1001 /usr/src/build/target/quarkus-app/quarkus/ /deployments/quarkus/

EXPOSE 8080
USER 1001

ENTRYPOINT [ "/deployments/run-java.sh" ]

