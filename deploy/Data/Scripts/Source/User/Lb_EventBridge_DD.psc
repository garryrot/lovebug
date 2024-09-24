Scriptname Lb_EventBridge_DD extends Quest

Event OnInit()
    RegisterForMagicEffectApplyEvent(Game.GetPlayer())
EndEvent

Event OnMagicEffectApply(ObjectReference akTarget, ObjectReference akCaster, MagicEffect akEffect)
    Debug.Trace(akCaster + " applied the " + akEffect.GetName() + " on " + akTarget)   
    Debug.Notification(akCaster + " applied the " + akEffect.GetName() + " on " + akTarget)

    If akEffect.GetName() == "Plugged"
        Lb_Native.Action("vibrate", 100, -1)
    EndIf

    RegisterForMagicEffectApplyEvent(Game.GetPlayer())
EndEvent

; OnItemEquipped
; OnItemUnequipped
