Scriptname TK2:Main extends Quest

Event OnInit()	
      Debug.Notification("Tele: OnInit")
	RegisterForRemoteEvent(Game.GetPlayer(), "OnPlayerLoadGame")
	RegisterEvents()
EndEvent

Event Actor.OnPlayerLoadGame(Actor ActorRef)
	Debug.Notification("Tele: OnPlayerLoadGame")
	RegisterEvents()
EndEvent

Function RegisterEvents()
    Debug("Registering Events")
	ScriptObject aafBridge = CastAs("TK2:AAF_EventBridge")
	If aafBridge
        Debug("AAF_EventBridge Exists")
		aafBridge.CallFunction("RegisterEvents", new Var[0])
    Else
        Debug("AAF_EventBridge Not Existing")
	EndIf

EndFunction

Function Debug(String msg)
    Debug.Notification(msg)
    Debug.Trace(msg)
EndFunction
