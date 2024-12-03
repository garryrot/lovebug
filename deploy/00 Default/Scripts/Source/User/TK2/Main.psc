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
		Debug("Starting AAF Event Bridge")
		aafBridge.CallFunction("Startup", new Var[0])
	EndIf
	
	ScriptObject ddBridge = CastAs("TK2:DD_EventBridge")
	If ddBridge
		Debug("Starting DD Event Bridge")
		ddBridge.CallFunction("Startup", new Var[0])
	EndIf
	Connect()
EndFunction

Function Connect()
	Int connection = MCM.GetModSettingInt("Telekinesis", "iType:Connection")
	String host = MCM.GetModSettingString("Telekinesis", "sHost:Connection")
	String port = MCM.GetModSettingString("Telekinesis", "sPort:Connection")
	Bool bluetooth = MCM.GetModSettingInt("Telekinesis", "bBluetooth:Connection")
	Bool xInput = MCM.GetModSettingInt("Telekinesis", "bXInput:Connection")
	Bool serial = MCM.GetModSettingInt("Telekinesis", "bSerial:Connection")
	; Debug.MessageBox(bluetooth + " xi " + xInput + " se " + serial)
	bool connected = Telekinesis.Connect(connection, port, host, bluetooth, xInput, serial)
EndFunction

Function Debug(String msg)
    Debug.Notification("[Tele] " + msg)
    Debug.Trace("[Tele] " + msg)
EndFunction