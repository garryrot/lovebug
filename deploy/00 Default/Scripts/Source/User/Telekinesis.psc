Scriptname Telekinesis extends ScriptObject Native

Bool Function Connect(Int connection, String port, String host, Bool bluetooth, Bool xinupt, Bool serial) Native Global
Int Function Process_Event(String eventName, String strArg, float floatArg) Native Global
Int Function Action(String actionName, Int speed, Float secs) Native Global
Int Function Scene(String sceneName, Actor[] actors, String[] tags, Int speed, Float secs) Native Global
Bool Function Update(Int handle, Int speed) Native Global
Bool Function Stop(Int handle) Native Global
Function Disconnect() Native Global

; MCM Models

Struct DevicePage
    Int Index = -1
    String Actuator = ""
    Bool Enabled = False
    Bool Anal = False
    Bool Clitoral = False
    Bool Nipple = False
    Bool Oral = False
    Bool Penis = False
    Bool Vaginal = False
EndStruct

Int Function MCM_Devices_Len() Native Global
DevicePage Function MCM_Devices_Get(Int index) Native Global
Bool Function MCM_Devices_Set(DevicePage actuator) Native Global
