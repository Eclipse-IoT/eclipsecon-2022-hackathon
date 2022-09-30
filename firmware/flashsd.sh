#!/bin/bash

SD_VERSION=7.2.0

# Download softdevice version from
# https://github.com/NordicSemiconductor/nRF5-SDK-for-Mesh/tree/master/bin/softdevice
curl -s https://raw.githubusercontent.com/NordicSemiconductor/nRF5-SDK-for-Mesh/master/bin/softdevice/s140_nrf52_${SD_VERSION}_softdevice.hex -o s140_nrf52_${SD_VERSION}_softdevice.hex

probe-rs-cli erase --chip nRF52833_xxAA
probe-rs-cli download s140_nrf52_${SD_VERSION}_softdevice.hex --format Hex --chip nRF52833_xxAA
