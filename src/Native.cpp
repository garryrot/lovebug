bool Connect(std::monostate, int connection, std::string port, std::string host, bool bluetooth, bool xinupt, bool serial)
{
    return lb_connect(connection, port, host, bluetooth, xinupt, serial);
}

int ProcessEvent(std::monostate, std::string eventName, std::string strArg, float numArg)
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
    lb_dynamic_stop();
    return lb_stop(handle);
}

void Disconnect(std::monostate)
{
    return lb_disconnect();
}

int Scene(std::monostate, std::string sceneName, std::vector<RE::Actor*> actors, std::vector<std::string> tags, int speed, float secs) 
{
    auto actorVec = ActorVec::ActorVec(actors);
    int x = lb_scene(sceneName, tags, speed, secs, actorVec);
    return x;
}

constexpr std::string_view TkClass = "Telekinesis";
bool RegisterPapyrusCalls(IVirtualMachine *vm)
{
    vm->BindNativeMethod(TkClass, "Connect", Connect, false);
    vm->BindNativeMethod(TkClass, "Process_Event", ProcessEvent, false);
    vm->BindNativeMethod(TkClass, "Action", Action, false);
    vm->BindNativeMethod(TkClass, "Update", Update, false);
    vm->BindNativeMethod(TkClass, "Stop", Stop, false);
    vm->BindNativeMethod(TkClass, "Scene", Scene, false);
    vm->BindNativeMethod(TkClass, "Disconnect", Disconnect, false);

    vm->BindNativeMethod(TkClass, "MCM_Devices_Get", MCM_Devices_Get, false);
    vm->BindNativeMethod(TkClass, "MCM_Devices_Set", MCM_Devices_Set, false);
    vm->BindNativeMethod(TkClass, "MCM_Devices_Len", MCM_Devices_Len, false);
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
