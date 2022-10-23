#!/bin/bash
NAME=$1

while true;
do
	out=$(probe-run --list-probes | grep Error)
	if [ "$out" == "" ]; then
		echo "Probe ready, flashing"
		pushd ../firmware
		./flashsd.sh
		popd

		./preprovision.sh $NAME

		pushd ../firmware
		cargo flash --release --no-default-features --features panic-reset --chip nRF52833_xxAA
		popd
		exit 0
	else
		echo "Probe not ready yet"
	fi
	sleep 1
done
