ScriptName TK2:AAF_EventBridge extends Quest

AAF:AAF_API aaf = None
TK2:MCM_Devices tkMcm = None

Int CurrentHandle = -1

Function Startup()
    aaf = Game.GetFormFromFile(0x00000F99, "AAF.esm") as AAF:AAF_API
    If (aaf)
        RegisterForCustomEvent(aaf, "OnAnimationStart")
        RegisterForCustomEvent(aaf, "OnAnimationStop")
        RegisterForCustomEvent(aaf, "OnAnimationChange")
    EndIf

    tkMcm = Game.GetFormFromFile(0x2665, "Telekinesis.esp") as TK2:MCM_Devices
    If ! tkMcm
        Debug.MessageBox("MCM not found")
        return
    EndIf

    tkMcm.AAF_Started = True
EndFunction

Event AAF:AAF_API.OnAnimationStart(AAF:AAF_API akSender, Var[] akArgs)
    If !HasFailed(akArgs) && HasPlayer(akArgs)
        StartScene(akArgs)
    EndIf
EndEvent

Event AAF:AAF_API.OnAnimationChange(AAF:AAF_API akSender, Var[] akArgs)
    If !HasFailed(akArgs) && HasPlayer(akArgs)
        StartScene(akArgs)
    EndIf
EndEvent

Event AAF:AAF_API.OnAnimationStop(AAF:AAF_API akSender, Var[] akArgs)
    If !HasFailed(akArgs) && HasPlayer(akArgs)
        StopScene(akArgs)
    EndIf
EndEvent

Bool Function HasPlayer(Var[] akArgs)
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
        Telekinesis.Stop(CurrentHandle)
    EndIf
    CurrentHandle = Telekinesis.Scene(sceneName, actors, tags, 100, -1)
EndFunction

Function StopScene(Var[] akArgs)
    String sceneName = akArgs[2] as String
    String[] tags = Utility.VarToVarArray(akArgs[3]) as String[]         
    Telekinesis.Stop(CurrentHandle)
    CurrentHandle = -1
EndFunction

Function Debug(String msg)
    Debug.Notification(msg)
    Debug.Trace(msg)
EndFunction
