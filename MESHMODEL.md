# Mesh models

The mesh models for the micro:bit and the JSON representation for each model message.


### Elements

#### 0

Model: GenericOnOffClient

For button 'A' on the micro:bit

#### 1

Model: GenericOnOffClient

For button 'B' on the micro:bit

#### 2

Model: GenericOnOffServer

For LED matrix. The behavior when on is defined by the application.

#### 3

Model: GenericBatteryServer

For measuring the battery level

#### 4

Model: SensorSetupServer

For reporting sensor data from peripherals


#### 5 

Model: GenericManufacturerPropertyServer

For reporting firmware and hardware revisions

## Messages

The JSON representation of the different BT mesh message types:


### GenericOnOffSet

    {
        "on": bool,
    } 

### GenericOnOffStatus

    {
        "on": bool,
    }

### GenericBatteryStatus

    {
        "level": u8,
        "flags": {
            "presence": "NotPresent" | "PresentRemovable",
        }
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
