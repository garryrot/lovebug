
bool ProcessEvent(std::monostate, std::string eventName, std::string strArg, float numArg)
{
    return lb_process_event(eventName, strArg, numArg);
}

int Action(std::monostate, std::string actionName, int speed, float secs) 
{
    return lb_action(actionName, speed, secs);
}

bool Update(std::monostate, int handle, int speed) 
{
    return lb_update(handle, speed);
}

bool Stop(std::monostate, int handle) 
{
    if (handle > -1)
    {
        boneThreadHandle = -1;
    }
    return lb_stop(handle);
}

int threadCounter = 0;
int Scene(std::monostate, std::string sceneName, std::vector<RE::Actor*> actors, std::vector<std::string> tags, int speed, float secs) 
{
    int x = lb_scene(sceneName, tags, speed, secs);
    if (boneThreadHandle == -1)
    {
        BoneThread[(threadCounter ++) % 128] = std::thread(Bone_Monitoring_Prototype, actors);
        boneThreadHandle = x;
    }
    return x;
}

constexpr std::string_view PapyrusClass = "Lb_Native";
bool RegisterPapyrusCalls(IVirtualMachine *vm)
{
    vm->BindNativeMethod(PapyrusClass, "Process_Event", ProcessEvent, false);
    vm->BindNativeMethod(PapyrusClass, "Action", Action, false);
    vm->BindNativeMethod(PapyrusClass, "Update", Update, false);
    vm->BindNativeMethod(PapyrusClass, "Stop", Stop, false);
    vm->BindNativeMethod(PapyrusClass, "Scene", Scene, false);
    return true;
}

void InitializeNative()
{
    lb_log_debug("Initializing Papyrus binding...");
    if (! F4SE::GetPapyrusInterface()->Register(RegisterPapyrusCalls))
    {
        lb_log_error("Failed binding papyrus");
    }
}
