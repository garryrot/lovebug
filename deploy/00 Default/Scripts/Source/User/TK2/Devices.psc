Scriptname TK2:Devices extends Quest

Event OnInit()
    RegisterForExternalEvent("Tele_DeviceAdded", "OnDeviceAdded")
    RegisterForExternalEvent("Tele_DeviceRemoved", "OnDeviceRemoved")
    RegisterForExternalEvent("Tele_ConnectionError", "OnConnectionError")
EndEvent

Function OnDeviceAdded(String strArg, Float numArg)
    Debug.Notification("[Tele] '" + strArg + "' connected.")
EndFunction

Function OnDeviceRemoved(String strArg, Float numArg)
    Debug.Notification("[Tele] '" + strArg + "' disconnected.")
EndFunction

Function OnConnectionError(String strArg, Float numArg)
    Debug.Notification("[Tele] Connection Error")
EndFunction