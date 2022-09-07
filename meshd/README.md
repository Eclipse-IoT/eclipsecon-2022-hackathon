# meshcfg

Running a mesh network on a Linux host with blueooth mesh daemon. 

To start the daemon:

```
mkdir -p ${PWD}/lib
sudo /usr/libexec/bluetooth/bluetooth-meshd --config ${PWD}/config --storage ${PWD}/lib --debug
```
