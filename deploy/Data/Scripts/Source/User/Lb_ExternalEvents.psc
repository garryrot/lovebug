Scriptname Lb_ExternalEvents extends Quest

Event OnInit()
    RegisterForExternalEvent("Tele_DeviceAdded", "OnDeviceAdded")
    RegisterForExternalEvent("Tele_DeviceRemoved", "OnDeviceRemoved")
    RegisterForExternalEvent("Tele_ConnectionError", "OnConnectionError")
EndEvent

Function OnDeviceAdded(String strArg, Float numArg)
    Debug.Notification("Device '" + strArg + "' connected.")
EndFunction

Function OnDeviceRemoved(String strArg, Float numArg)
    Debug.Notification("Device '" + strArg + "' disconnected.")
EndFunction

Function OnConnectionError(String strArg, Float numArg)
    Debug.Notification("Connection error: '" + strArg + "'")
EndFunction