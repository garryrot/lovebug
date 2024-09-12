Scriptname Lb_ExternalEvents extends Quest

Event OnInit()
    RegisterForExternalEvent("LbEvent", "OnDeviceEvent")
EndEvent

Function OnDeviceEvent(String strArg, Float numArg)
    Debug.Notification("Lovebug: " + strArg)
EndFunction
