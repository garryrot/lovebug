Scriptname Lb_EventBridge_AAF extends Quest

Int CurrentHandle = -1

Event OnInit()
    Debug("Lb_EventBridge_AAF OnInit")

    AAF:AAF_API api = Game.GetFormFromFile(0x00000F99, "AAF.esm") as AAF:AAF_API
    RegisterForCustomEvent( api, "OnAnimationStart" )
    RegisterForCustomEvent( api, "OnAnimationStop" )
    RegisterForCustomEvent( api, "OnAnimationChange" )
EndEvent

Event AAF:AAF_API.OnAnimationStart(AAF:AAF_API akSender, Var[] akArgs)
    int status = akArgs[0] as int
    If status != 0
        Debug("OnAnimationStart Failed " + status)
	    return
    EndIf

    Actor[] actors = Utility.VarToVarArray(akArgs[1]) as Actor[]
    Int i = actors.Length
    While i > 0
        i -= 1
        If actors[ i ] == Game.GetPlayer()     
            String sceneName = akArgs[2] as String ; Animation/Scene Name
            String[] tags = Utility.VarToVarArray(akArgs[3]) as String[]         
            Debug("Lb_Native.Scene " + status + " sceneName:" + sceneName + " tags: " + tags)
            CurrentHandle = Lb_Native.Scene(sceneName, tags, 100, 60)
        EndIf
    EndWhile

EndEvent

Event AAF:AAF_API.OnAnimationStop(AAF:AAF_API akSender, Var[] akArgs)
    int status = akArgs[0] as int
    If status != 0
        Debug("OnAnimationStop Failed " + status)
	    return
    EndIf

    Actor[] actors = Utility.VarToVarArray(akArgs[1]) as Actor[]
    Int i = actors.Length
    While i > 0
        i -= 1
        If actors[ i ] == Game.GetPlayer()     
            String sceneName = akArgs[2] as String ; Animation/Scene Name
            String[] tags = Utility.VarToVarArray(akArgs[3]) as String[]         
            Debug("Lb_Native.Stop " + status + " sceneName:" + sceneName + " tags: " + tags)
            Lb_Native.Stop(CurrentHandle)
            CurrentHandle = -1
        EndIf
    EndWhile
EndEvent

Event AAF:AAF_API.OnAnimationChange(AAF:AAF_API akSender, Var[] akArgs)
    int status = akArgs[0] as int
    If status != 0
        Debug("OnAnimationChange Failed " + status)
	    return
    EndIf

    Actor[] actors = Utility.VarToVarArray(akArgs[1]) as Actor[]
    String position = akArgs[2] as String
    String[] tags = Utility.VarToVarArray(akArgs[3]) as String[]

    Debug("OnAnimationChange" + status + " actors:" + actors +  " tags: " + tags )
EndEvent

Function Debug(String msg)
    Debug.Notification(msg)
    Debug.Trace(msg)
EndFunction