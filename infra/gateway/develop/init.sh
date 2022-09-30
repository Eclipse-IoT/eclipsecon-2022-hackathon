#!/bin/bash

rm -rf /tmp/meshd/
mkdir /tmp/meshd

cp -r ../meshd/config /tmp/meshd/config
cp -r ../meshd/lib /tmp/meshd/lib

find /tmp/meshd
