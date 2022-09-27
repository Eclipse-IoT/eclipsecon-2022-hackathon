# OPC UA Server

## Creating a server key

```shell
APPLICATION_URI=https://drogue.io/doppelgaenger/opcua
openssl req -newkey rsa:4096 -nodes -keyout server.key -x509 -days 365 -out server.crt -subj "/CN=Drogue IoT/O=Red Hat, Inc" -sha256 -addext "subjectAltName = URI:${APPLICATION_URI}" 
```

Show content:

```shell
openssl x509 -in server.crt -text
```

Convert to configmap:

```shell
kubectl create secret tls doppelgaenger-opcua-server-key --cert=server.crt --key=server.key --dry-run=client -o yaml > server-key.yaml
```