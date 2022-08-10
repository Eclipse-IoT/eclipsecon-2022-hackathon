# Mesh models

The mesh models for the micro:bit and the JSON messages converted for each model:

element 0:
    GenericBatteryServer
        For measuring the battery level
        json:
        ```
        {
            "level": u8,
            "flags": {
                "presence": "NotPresent" | "PresentRemovable",
            }
        }
        ```

element 1:
    GenericOnOffClient
        For button 'A' on the micro:bit
        json:
        ```
        {
            "pressed": bool,
        } 
        ```

element 2:
    GenericOnOffClient
        For button 'B' on the micro:bit
        json:
        ```
        {
            "pressed": bool,
        } 
        ```

element 3:
    SensorServer
        Status:
            Temperature:
                Property ID: 0x4F
                Value: 
            Noise:
                Property ID: 0x79
                Value: Generic Level
            Motion Sensed:
                Property ID: 0x42
                Value: Percentage 8
        json:
        ```
        {
            "temperature": {
                "value": u8
            },
            "noise": {
                "value": u8
            }
            "motion": {
                "sensed": u8
            }
        }
        ```
        
        SettingSet: 
            Motion Threshold:
                Property ID: 0x43
                Value: Percentage 8
        json:
        ```
        {
            "motion": {
                "threshold": u8
            }
        }
        ```
element 4:
    GenericManufacturerPropertyServer
        Properties:
            DeviceSerialNumber:
                Property ID: 0x19
                Value: Fixed String 16
            DeviceFirmwareRevision:
                Property ID: 0x0E
                Value: Fixed String 8
         json:
         ```
        {
            "device": {
                "serialNumber": "1234",
                "firmwareRevision": "a",
            }
        }
         ```
        
