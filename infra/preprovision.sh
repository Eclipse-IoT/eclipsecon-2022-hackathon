#!/usr/bin/env bash

CLAIM=$1
if [ "$CLAIM" == "" ]; then
    echo "Invalid claim ${CLAIM}"
    exit 1
fi

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

IDMAP="$SCRIPT_DIR/../example-apps/console/src/main/resources/META-INF/resources/idmap.json"

address=$(jq ".[] | select(.id==\"${CLAIM}\").address" ${IDMAP} | xargs)
devkey=$(jq ".[] | select(.id==\"${CLAIM}\").devkey" ${IDMAP} | xargs)

if [ "$address" != "" ] && [ "$devkey" != "" ]
then
    echo "Provisioning with $address and key $devkey"
    pre-provision provision --flash-address 0x7E000 --node-address 0x${address} --network-key 0B5E6760156116BAB83115D4C1BFB480 --application-key 8E0A245C38A136E7D6E8429D562DA959 --device-key ${devkey} --chip nRF52833_xxAA
    pre-provision provision --flash-address 0x7F000 --node-address 0x${address} --network-key 0B5E6760156116BAB83115D4C1BFB480 --application-key 8E0A245C38A136E7D6E8429D562DA959 --device-key ${devkey} --chip nRF52833_xxAA
    # TODO Generate drg config and run drg apply
else
    echo "No address and key found for claim ${CLAIM}"
fi
