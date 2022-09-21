FROM docker.io/fedora:36

RUN dnf install -y bluez-mesh
RUN rpm -ql bluez-mesh

ENTRYPOINT [ "/usr/libexec/bluetooth/bluetooth-meshd" ]