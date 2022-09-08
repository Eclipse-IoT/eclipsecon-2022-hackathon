#!/bin/sh
ARGS=""


ARGS="${ARGS} --mqtt-uri ssl://${DROGUE_MQTT_INTEGRATION}"
ARGS="${ARGS} --token ${DROGUE_TOKEN}"
ARGS="${ARGS} --user ${DROGUE_USER}"
ARGS="${ARGS} --device-registry ${DROGUE_DEVICE_REGISTRY}"
ARGS="${ARGS} --application ${DROGUE_APPLICATION}"

if [ "${MQTT_GROUP_ID}" != "" ]; then
    ARGS="${ARGS} --mqtt-group-id ${MQTT_GROUP_ID}"
fi

if [ "${RECONCILE_INTERVAL}" != "" ]; then
    ARGS="${ARGS} --interval ${RECONCILE_INTERVAL}"
fi

/btmesh-operator ${ARGS}
