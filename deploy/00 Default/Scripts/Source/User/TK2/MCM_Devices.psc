Scriptname TK2:MCM_Devices  extends Quest

String Property Pagination = "" Auto

Int CurrentIndex = -1

Int Property Index = -1 Auto
String Property Actuator = "" Auto
Bool Property Enabled = False Auto
Bool Property Anal = True Auto
Bool Property Clitoral = True Auto
Bool Property Nipple = True Auto
Bool Property Oral = True Auto
Bool Property Penis = True Auto
Bool Property Vaginal = True Auto

Bool Property AAF_Started = False Auto
Bool Property AAF_Not_Started = True Auto

Bool Property DD_Started = False Auto
Bool Property DD_Not_Started = False Auto

Bool Property Connected = False Auto 
Bool Property ConnectionError = True Auto

Telekinesis:DevicePage current = none

Event OnInit()
    RegisterForExternalEvent("OnMCMSettingChange|Telekinesis", "OnChange")
    RegisterForExternalEvent("Tele_DeviceAdded", "OnDeviceAdded")
    RegisterForExternalEvent("Tele_DeviceRemoved", "OnDeviceRemoved")
    RegisterForExternalEvent("Tele_ConnectionSuccess", "OnConnectionSuccess")
    RegisterForExternalEvent("Tele_ConnectionError", "OnConnectionError")
    Telekinesis:DevicePage default
    current = default

	RegisterForRemoteEvent(Game.GetPlayer(), "OnPlayerLoadGame")
    Startup()
EndEvent

Event Actor.OnPlayerLoadGame(Actor ActorRef)
	Startup()
EndEvent

Function OnDeviceAdded(String strArg, Float numArg)
   Startup()
EndFunction

Function OnDeviceRemoved(String strArg, Float numArg)
    Startup()
EndFunction 

Function OnConnectionSuccess(String strArg, Float numArg)
    Connected = True 
    ConnectionError = False
EndFunction

Function OnConnectionError(String strArg, Float numArg)
    Connected = False
    ConnectionError = True
EndFunction

Function Reconnect()
	TK2:Main main = Game.GetFormFromFile(0x1732, "Telekinesis.esp") as TK2:Main
	Telekinesis.Disconnect()
    Connected = False
    ConnectionError = False
	MCM.RefreshMenu()
	Debug.MessageBox("Reconnecting")
	Utility.Wait(1.0)
	main.Connect()
	MCM.RefreshMenu()
EndFunction

Function Startup()
    int len = Telekinesis.MCM_Devices_Len()
    CurrentIndex = 0
    ConnectionError = False
    Update(len)
EndFunction

Function Next()
    int len = Telekinesis.MCM_Devices_Len()
    If CurrentIndex + 1 < len
        CurrentIndex += 1
    EndIf
    Update(len)
EndFunction

Function Prev()
    int len = Telekinesis.MCM_Devices_Len()
    If CurrentIndex > 0
        CurrentIndex -= 1
    EndIf
    Update(len)
EndFunction

Function Update(int len)
    If len < 1
        Pagination = "No devices loaded..."
    Else
        Pagination = (CurrentIndex + 1) + " / " + len
    EndIf

    If CurrentIndex >= 0
        Telekinesis:DevicePage x = Telekinesis.MCM_Devices_Get(CurrentIndex)
        current = x
        If x.Index != -1
            Index = x.Index
            Actuator = x.Actuator
            Enabled = x.Enabled
            Anal = x.Anal
            Clitoral = x.Clitoral
            Nipple = x.Nipple
            Oral = x.Oral
            Penis = x.Penis
            Vaginal = x.Vaginal
        Else
            Index = -1
            Actuator = ""
            Enabled = False
            Anal = False
            Clitoral = False 
            Nipple = False            
            Oral = False            
            Penis = False             
            Vaginal = False 
        EndIf
    EndIf

    MCM.RefreshMenu()
EndFunction

Function OnChange(string modName, string id)
    If current != None
        If (id == "enabled:Device" || id == "anal:Device" || id == "clit:Device" || id == "nipple:Device" || id == "oral:Device" ||  id == "vaginal:Device" || id == "penis:Device")
            current.Enabled = Enabled
            current.Anal = Anal
            current.Clitoral = Clitoral
            current.Nipple = Nipple
            current.Oral = Oral
            current.Penis = Penis
            current.Vaginal = Vaginal
            Telekinesis.MCM_Devices_Set(current)
        EndIf
    EndIf
EndFunction
