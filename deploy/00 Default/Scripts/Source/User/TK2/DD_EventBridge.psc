ScriptName TK2:DD_EventBridge extends Quest

DD:DD_Library libs = None
TK2:MCM_Devices tkMcm = None

; Inflate
Int plugInflateHandle = -1
; Vibrate
Int plugVibrateHandle = -1

Function Startup()
    libs = Game.GetFormFromFile(0x00004C50, "Devious Devices.esm") as DD:DD_Library
    If ! libs
        Debug.MessageBox("DD Library not found")
        return
    EndIf

    tkMcm = Game.GetFormFromFile(0x2665, "Telekinesis.esp") as TK2:MCM_Devices
    If ! tkMcm
        Debug.MessageBox("MCM not found")
        return
    EndIf

    plugInflateHandle = -1
    plugVibrateHandle = -1
    tkMcm.DD_Started = True

    RegisterForRemoteEvent(Game.GetPlayer(), "OnItemEquipped")

    ; StartTimer(5.0)

    StartEvents()
EndFunction

; Event OnTimer(Int TimerID)
;     DumpPlayerStatus()
;     StartTimer(5.0)
; EndEvent

; Function DumpPlayerStatus()
;     Actor player = Game.GetPlayer()
;     Debug.Trace("Player.IsVibrating " + player.HasKeyword(libs.DD_kw_Event_IsVibrating))
;     Debug.Trace("Player.InflateVaginal " + player.GetValue(libs.DD_AV_InflateStatusVaginal))
;     Debug.Trace("Player.InflateAnal    " + player.GetValue(libs.DD_AV_InflateStatusAnal))
;     Debug.Trace("Player.VibrateVaginal " + player.GetValue(libs.DD_AV_VibrateStrengthVaginal))
;     Debug.Trace("Player.VibrateAnal    " + player.GetValue(libs.DD_AV_VibrateStrengthAnal))
;     Debug.Trace("Player Keywords: " + player.GetKeywords())
; EndFunction

Event Actor.OnItemEquipped(Actor sender, Form object, ObjectReference reference)
    Armor item = object as Armor
    Debug.Trace("Item.GetKeywords: : " + item.GetKeywords())
    If item && item.HasKeyword( libs.DD_kw_RenderedItem )
        StartEvents()
    EndIf
EndEvent

Function StartEvents()
    Actor player = Game.GetPlayer()
    Debug("Initializing Events")
    Debug.Trace("Player Keywords: " + player.GetKeywords())
    If plugInflateHandle == -1
        If player.WornHasKeyword(libs.DD_kw_ItemEffect_PlugInflate)
            plugInflateHandle = Telekinesis.Process_Event("dd.inflate", "", 0.0)
        EndIf
    EndIf
EndFunction

Function Debug(String msg)
    Debug.Notification("[Tele] " + msg)
    Debug.Trace("[Tele] " + msg)
EndFunction