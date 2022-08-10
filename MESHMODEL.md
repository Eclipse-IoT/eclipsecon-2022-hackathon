# Mesh models

The mesh models for the micro:bit are defined as follows:

element 0:
    GenericBatteryServer
        For measuring the battery level

element 1:
    GenericOnOffClient
        For button 'A' on the micro:bit

element 2:
    GenericOnOffClient
        For button 'B' on the micro:bit

element 3:
    SensorServer
        Data:
            Temperature:
                Property ID: 0x4F
                Value: 
            Noise:
                Property ID: 0x79
                Value: Generic Level
            Motion Sensed:
                Property ID: 0x42
                Value: Percentage 8
        
        Settings: 
            Motion Threshold:
                Property ID: 0x43
                Value: Percentage 8
element 4:
    GenericManufacturerPropertyServer
        Properties:
            DeviceSerialNumber:
                Property ID: 0x19
                Value: Fixed String 16
            DeviceFirmwareRevision:
                Property ID: 0x0E
                Value: Fixed String 8
        

## Converted to JSON

{
}
