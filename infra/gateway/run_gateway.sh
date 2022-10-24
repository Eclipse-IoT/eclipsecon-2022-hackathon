#!/bin/bash
GW=$1
rm -rf /tmp/meshd
cp -r /opt/meshd /tmp/meshd
/opt/gateway/eclipsecon-gateway --drogue-device $GW --drogue-application eclipsecon-hackathon --token dd26596e54e78fa2 --ca-path /etc/ssl/certs/ca-certificates.crt
