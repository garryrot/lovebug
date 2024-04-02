Scriptname Lb_Bridge extends Quest

Function BridgeEvents()
    ; Bridge non-default event signatures to be usable by Lovebug
	RegisterForModEvent("MilkQuest.StopMilkingMachine",  "OnStopMilkingMachine")	
	RegisterForModEvent("MilkQuest.FuckMachineStage", "OnFuckMachineStage")
	RegisterForModEvent("MilkQuest.MilkingStage",  "OnMilkingStage")
    RegisterForModEvent("MilkQuest.StartMilkingMachine", "OnStartMilkingMachine")
	RegisterForModEvent("MilkQuest.FeedingStage", "OnFeedingStage")
EndFunction 

Event OnStopMilkingMachine(Form sender, Int mpas, Int milkingType)
    Lb_Native.Process_Event("MilkQuest.StopMilkingMachine", "", 0.0)
EndEvent

Event OnFuckMachineStage(Form sender, Int mpas, Int MilkingType)
    Lb_Native.Process_Event("MilkQuest.FuckMachineStage", "", 0.0)
EndEvent

Event OnMilkingStage(Form sender, Int mpas, Int MilkingType)
    Lb_Native.Process_Event("MilkQuest.MilkingStage", "", 0.0)
EndEvent

Event OnStartMilkingMachine(Form sender, int mpas, int MilkingType)
    Lb_Native.Process_Event("MilkQuest.StartMilkingMachine", "", 0.0)
EndEvent

Event OnFeedingStage(Form sender, Int mpas, Int MilkingType)
    Lb_Native.Process_Event("MilkQuest.FeedingStage", "", 0.0)
EndEvent
