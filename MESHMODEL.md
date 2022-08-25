# Mesh models

The mesh models for the micro:bit and the JSON representation for each model message.


### Elements

#### front

Model: GenericOnOffServer

For LED matrix. The behavior when on is defined by the application.

Model: GenericBatteryServer

For measuring the battery level


Model: SensorSetupServer

For reporting sensor data from peripherals


Model: GenericManufacturerPropertyServer

For reporting firmware and hardware revisions

#### left

Model: GenericOnOffClient

For button 'A' on the micro:bit

#### right

Model: GenericOnOffClient

For button 'B' on the micro:bit


## Messages

The JSON representation of the different BT mesh message types follows. The "location" field matches the BT mesh location of the element for the model.


### GenericOnOffSet

    {
        "on": bool,
        "location": 0,
    } 

### GenericOnOffStatus

    {
        "on": bool,
        "location": 0,
    }

### GenericBatteryStatus

    {
        "level": u8,
        "flags": {
            "presence": "NotPresent" | "PresentRemovable",
        }
        "location": 0,
    }

### SensorStatus

Properties:

* Temperature
  * Property ID: 0x4F
  * Value: Temperature 8
* Noise
  * Property ID: 0x79
  * Value: Generic Level

* Motion Sensed
  * Property ID: 0x42
  * Value: Percentage 8
  

```
{
    "temperature": {
        "value": i8
    },
    "noise": {
        "value": u8
    }
    "motion": {
        "sensed": u8
    }
    "location": 0,
}
```        
        
### SensorSettingSet

Properties:

* Motion Threshold
  * Property ID: 0x43
  * Value: Percentage 8


```        
{
    "motion": {
        "threshold": u8
    }
}
```        

### GenericManufacturerPropertyStatus

Properties:

* DeviceSerialNumber
  * Property ID: 0x19
  * Value: Fixed String 16
* DeviceFirmwareRevision
  * Property ID: 0x0E
  * Value: Fixed String 8

```
{
    "properties": {
        "serialNumber": "1234",
        "firmwareRevision": "a",
    }
}
```
