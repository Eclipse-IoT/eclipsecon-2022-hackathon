# Bootstrap

## Setup

* https://docs.openshift.com/container-platform/4.10/monitoring/enabling-monitoring-for-user-defined-projects.html

## Operators

Install the following operators:

* Strimzi / Red Hat Integration - AMQ Streams
* Jaeger / Red Hat OpenShift distributed tracing platform

## CRDs

Install CRDs:

```shell
oc apply -f https://raw.githubusercontent.com/keycloak/keycloak-k8s-resources/18.0.1/kubernetes/keycloaks.k8s.keycloak.org-v1.yml
oc apply -f https://raw.githubusercontent.com/keycloak/keycloak-k8s-resources/18.0.1/kubernetes/keycloakrealmimports.k8s.keycloak.org-v1.yml
```

Bootstrap setup:

```shell
oc apply -f bootstrap.yaml 
```
