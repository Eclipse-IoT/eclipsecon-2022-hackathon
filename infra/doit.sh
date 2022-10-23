#!/bin/bash
NAME=$1

pushd ../firmware
./flashsd.sh
cargo flash --release --chip nRF52833_xxAA
popd

./preprovision.sh $NAME
