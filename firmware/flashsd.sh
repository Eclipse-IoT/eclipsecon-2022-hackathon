#!/bin/bash
probe-rs-cli erase --chip nRF52833_xxAA
probe-rs-cli download s140_nrf52_7.3.0_softdevice.hex --format Hex --chip nRF52833_xxAA
