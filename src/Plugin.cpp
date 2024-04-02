#include <stddef.h>

#include <thread>
#include <chrono>
#include <stdlib.h> //for using the function sleep

#include "lbug/src/logging.rs.h"
#include "lbug/src/lib.rs.h"

using namespace RE;
using namespace RE::BSScript;
using namespace SKSE::log;
using namespace SKSE::stl;
using namespace SKSE;
using namespace REL;

#define DllExport __declspec(dllexport)
#define SFT StaticFunctionTag *


class ModEventSink : public RE::BSTEventSink<SKSE::ModCallbackEvent>
{
public:
    static ModEventSink *GetSingleton()
    {
        static ModEventSink singleton;
        return &singleton;
    }

    RE::BSEventNotifyControl ProcessEvent(const SKSE::ModCallbackEvent *event, RE::BSTEventSource<SKSE::ModCallbackEvent> *eventSource)
    {
        if (event == NULL || event->sender == NULL)
        {
            return RE::BSEventNotifyControl::kContinue;
        }
        lb_process_event(event, event->sender);
        return RE::BSEventNotifyControl::kContinue;
    }
};

// Native Events
bool Lb_Process_Event_Bridge(SFT, std::string eventName, std::string strArg, float numArg) // , RE::TESForm* sender
{
    return lb_process_event_bridge(eventName, strArg, numArg);
}

constexpr std::string_view PapyrusClass = "Lb_Native";
bool RegisterPapyrusCalls(IVirtualMachine *vm)
{
    vm->RegisterFunction("Process_Event", PapyrusClass, Lb_Process_Event_Bridge);
    return true;
}

void InitializePapyrus()
{
    lb_log_debug("Initializing Papyrus binding...");
    if (! GetPapyrusInterface()->Register(RegisterPapyrusCalls))
    {
        lb_log_error("Failed binding papyrus");
    }
}

// Messaging

void InitializeMessaging()
{
    if (!GetMessagingInterface()->RegisterListener([](MessagingInterface::Message *message)
                                                   {
            switch (message->type) {
                case MessagingInterface::kInputLoaded:
                    SKSE::GetModCallbackEventSource()->AddEventSink(ModEventSink::GetSingleton());
                    break;
                case MessagingInterface::kDataLoaded:
                    // All ESM/ESL/ESP plugins are loaded, forms can be used
                    lb_init();
                    break;
            } }))
    {
        lb_log_info("Failed registering message interface");
    }
}

std::string GetLogFile()
{
    auto path = log_directory();
    if (!path)
    {
        report_and_fail("Unable to lookup SKSE logs directory.");
    }
    return std::format("{}\\{}.log", path->string(), PluginDeclaration::GetSingleton()->GetName());
}

SKSEPluginLoad(const LoadInterface *skse)
{
    lb_init_logging(::rust::String(GetLogFile()));

    auto *plugin = PluginDeclaration::GetSingleton();
    auto version = plugin->GetVersion();
    lb_log_info(std::format("{} {} is loading...", plugin->GetName(), version));

    Init(skse);
    InitializePapyrus();
    InitializeMessaging();

    lb_log_info(std::format("{} has finished loading.", plugin->GetName()));
    return true;
}
