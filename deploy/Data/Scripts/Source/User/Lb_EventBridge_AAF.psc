Scriptname Lb_EventBridge_AAF extends Quest

Int CurrentHandle = -1

Event OnInit()
    AAF:AAF_API api = Game.GetFormFromFile(0x00000F99, "AAF.esm") as AAF:AAF_API
    RegisterForCustomEvent(api, "OnAnimationStart")
    RegisterForCustomEvent(api, "OnAnimationStop")
    RegisterForCustomEvent(api, "OnAnimationChange")
EndEvent

Event AAF:AAF_API.OnAnimationStart(AAF:AAF_API akSender, Var[] akArgs)
    Debug("AAF:AAF_API.OnAnimationStart")
    If !HasFailed(akArgs) && HasPlayer(akArgs)
        StartScene(akArgs)
    EndIf
EndEvent

Event AAF:AAF_API.OnAnimationChange(AAF:AAF_API akSender, Var[] akArgs)
    Debug("AAF:AAF_API.OnAnimationChange")
    If !HasFailed(akArgs) && HasPlayer(akArgs)
        StartScene(akArgs)
    EndIf
EndEvent

Event AAF:AAF_API.OnAnimationStop(AAF:AAF_API akSender, Var[] akArgs)
    Debug("AAF:AAF_API.OnAnimationStop")
    If !HasFailed(akArgs) && HasPlayer(akArgs)
        StopScene(akArgs)
    EndIf
EndEvent

Bool Function HasPlayer(Var[] akArgs)
    Debug("HasPlayer")
    Actor[] actors = Utility.VarToVarArray(akArgs[1]) as Actor[]
    Int i = actors.Length
    While i > 0
        i -= 1
        If actors[ i ] == Game.GetPlayer()
            return True
        EndIf
    EndWhile
    return False
EndFunction

Bool Function HasFailed(Var[] akArgs)
    Debug("HasFailed")
    int status = akArgs[0] as int
    If status != 0
        Debug("Scene Failed: " + status)
	    return True
    EndIf
    return False
EndFunction


Function StartScene(Var[] akArgs)
    String sceneName = akArgs[2] as String
    String[] tags = Utility.VarToVarArray(akArgs[3]) as String[]         
    If CurrentHandle != -1
        Lb_Native.Stop(CurrentHandle)
    EndIf
    CurrentHandle = Lb_Native.Scene(sceneName, tags, 100, -1)
EndFunction

Function StopScene(Var[] akArgs)
    String sceneName = akArgs[2] as String
    String[] tags = Utility.VarToVarArray(akArgs[3]) as String[]         
    Lb_Native.Stop(CurrentHandle)
    CurrentHandle = -1
EndFunction

Function Debug(String msg)
    Debug.Notification(msg)
    Debug.Trace(msg)
EndFunction