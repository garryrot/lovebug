ScriptName TK2:DD_EventBridge extends Quest

DD:DD_Library libs = None
TK2:MCM_Devices tkMcm = None

Bool started = False

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

    If libs && ! started
        Actor player = Game.GetPlayer()
        RegisterForRemoteEvent(player, "OnItemEquipped")
        started = True
    EndIf

    ; StartEvents()
EndFunction

Event Actor.OnItemEquipped(Actor sender, Form object, ObjectReference reference)
    Armor item = object as Armor
    If item && item.HasKeyword( libs.DD_kw_RenderedItem )
        ; StartEvents()
    EndIf
EndEvent

Function StartEvents()
    Actor player = Game.GetPlayer()

    If plugInflateHandle == -1
        If player.WornHasKeyword(libs.DD_kw_ItemEffect_PlugInflate)
            plugInflateHandle = Telekinesis.Process_Event("DD.Inflator", "", 0.0)
        EndIf
    EndIf

    If plugVibrateHandle == -1
        If player.WornHasKeyword(libs.DD_kw_ItemEffect_PlugVibrate)
            plugVibrateHandle = Telekinesis.Process_Event("DD.Vibrator", "", 0.0)
        ElseIf player.WornHasKeyword(libs.DD_kw_ItemEffect_PlugVibrate_EdgeOnly)
            plugVibrateHandle = Telekinesis.Process_Event("DD.Vibrator.Edge", "", 0.0)
        EndIf
    EndIf
EndFunction
