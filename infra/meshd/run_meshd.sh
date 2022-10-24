#!/bin/bash
rm -rf /tmp/meshd
cp -r /opt/meshd /tmp/meshd
/usr/libexec/bluetooth/bluetooth-meshd --config /tmp/meshd/config --storage /tmp/meshd/lib --nodetach
