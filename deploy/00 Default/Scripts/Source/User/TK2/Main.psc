ScriptName TK2:Main extends Quest

Event OnInit()	
    Debug.Notification("Telekinesis loaded. Make sure to enable devices in MCM...")
	RegisterForRemoteEvent(Game.GetPlayer(), "OnPlayerLoadGame")
	Startup()
EndEvent

Event Actor.OnPlayerLoadGame(Actor ActorRef)
	Startup()
EndEvent

Function Startup()
	ScriptObject aafBridge = CastAs("TK2:AAF_EventBridge")
	If aafBridge
		Trace("Starting AAF Event Bridge")
		aafBridge.CallFunction("Startup", new Var[0])
	EndIf
	
	ScriptObject ddBridge = CastAs("TK2:DD_EventBridge")
	If ddBridge
		Trace("Starting DD Event Bridge")
		ddBridge.CallFunction("Startup", new Var[0])
	EndIf

	RegisterForExternalEvent("Tele_Action", "OnAction")
    RegisterForExternalEvent("Tele_Event", "OnEvent")
    RegisterForExternalEvent("Tele_Scene", "OnScene")
	Connect()
EndFunction

Function OnAction(String description, Float handle)
	If MCM.GetModSettingInt("Telekinesis", "bDebug:Actions")
		Debug("Action: " + description + " " + handle)
	EndIf
EndFunction

Function OnEvent(String description, Float handle)
	If MCM.GetModSettingInt("Telekinesis", "bDebug:Scenes")
		Debug("Triggered Scene: " + description + " " + handle)
	EndIf
EndFunction

Function OnScene(String description, Float handle)
	If MCM.GetModSettingInt("Telekinesis", "bDebug:Events")
		Debug("Triggered Event: " + description + " " + handle)
	EndIf
EndFunction

Function Connect()
	Int connection = MCM.GetModSettingInt("Telekinesis", "iType:Connection")
	String host = MCM.GetModSettingString("Telekinesis", "sHost:Connection")
	String port = MCM.GetModSettingString("Telekinesis", "sPort:Connection")
	Bool bluetooth = MCM.GetModSettingInt("Telekinesis", "bBluetooth:Connection")
	Bool xInput = MCM.GetModSettingInt("Telekinesis", "bXInput:Connection")
	Bool serial = MCM.GetModSettingInt("Telekinesis", "bSerial:Connection")
	bool connected = Telekinesis.Connect(connection, port, host, bluetooth, xInput, serial)
EndFunction

Function Debug(String msg)
    Debug.Notification("[Tele] " + msg)
    Debug.Trace("[Tele] " + msg)
EndFunction