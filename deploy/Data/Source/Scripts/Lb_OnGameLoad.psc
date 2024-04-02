Scriptname Lb_OnGameLoad extends ReferenceAlias  

Event OnInit()
    Debug.MessageBox("Lb_GameLoad Init")
    Lb_Bridge bridge = GetOwningQuest() as Lb_Bridge
    bridge.BridgeEvents()
EndEvent

Event OnPlayerLoadGame()
    Debug.MessageBox("Lb_GameLoad OnPlayerLoadGame")
    Lb_Bridge bridge = GetOwningQuest() as Lb_Bridge
    bridge.BridgeEvents()
EndEvent
