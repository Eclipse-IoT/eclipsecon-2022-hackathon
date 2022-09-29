#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

IDMAP="$SCRIPT_DIR/../example-apps/console/src/main/resources/META-INF/resources/idmap.json"

# shellcheck disable=SC2089
device='{ "metadata": {
               "application": "eclipsecon-hackathon",
               "name": "",
               "labels": {
                 "role": "node"
              }
             },
             "spec": {
               "alias": [],
               "gatewaySelector": {
                 "matchNames": [
                   "gateway1",
                   "gateway2",
                   "gateway3",
                   "gateway4",
                   "gateway5"
                 ]
               }
             }
           }'

for row in $( jq -c '.[]' "${IDMAP}"); do
    ID=$(echo "$row" | jq -r ".id")
    ADDRESS=$(echo "$row" | jq -r ".address")
    LEFT=$(echo "$row" | jq -r ".left")
    RIGHT=$(echo "$row" | jq -r ".right")

    # shellcheck disable=SC2090
    JSON=$(echo "$device" | jq ".metadata.name = \"$ID\" | .spec.alias += [\"$ADDRESS\",\"$LEFT\",\"$RIGHT\"]")

    echo "$JSON" | jq -c | drg apply -f -
done

