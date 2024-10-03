Scriptname Lb_AnimationBridge extends Quest

Event OnInit()
    Debug.Notification("Lb_AnimationBridge")
    RegisterForAnimationEvent(Game.GetPlayer(), "WeaponFire")
EndEvent

Event OnAnimationEvent(ObjectReference akSource, string asEventName)
    Debug.Notification("WeaponFire")
EndEvent
