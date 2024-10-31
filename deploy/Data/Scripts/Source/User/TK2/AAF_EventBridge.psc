ScriptName TK2:AAF_EventBridge extends Quest

Int CurrentHandle = -1

Function RegisterEvents()
    Debug("RegisteringEvents")
    AAF:AAF_API aaf = Game.GetFormFromFile(0x00000F99, "AAF.esm") as AAF:AAF_API
    If (aaf)
        Debug("aaf exists")
        RegisterForCustomEvent(aaf, "OnAnimationStart")
        RegisterForCustomEvent(aaf, "OnAnimationStop")
        RegisterForCustomEvent(aaf, "OnAnimationChange")
    Else
        Debug.Notification("ERROR AAF NOT FOUND")
    EndIf
EndFunction

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
    Actor[] actors = Utility.VarToVarArray(akArgs[1]) as Actor[]
    String[] tags = Utility.VarToVarArray(akArgs[3]) as String[]         
    If CurrentHandle != -1
        TK2:Telekinesis.Stop(CurrentHandle)
    EndIf
    CurrentHandle = TK2:Telekinesis.Scene(sceneName, actors, tags, 100, -1)
EndFunction

Function StopScene(Var[] akArgs)
    String sceneName = akArgs[2] as String
    String[] tags = Utility.VarToVarArray(akArgs[3]) as String[]         
    TK2:Telekinesis.Stop(CurrentHandle)
    CurrentHandle = -1
EndFunction

Function Debug(String msg)
    Debug.Notification(msg)
    Debug.Trace(msg)
EndFunction
