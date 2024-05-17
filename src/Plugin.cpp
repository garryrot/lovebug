#include <stddef.h>

#include <thread>
#include <chrono>
#include <stdlib.h> //for using the function sleep

#include "lbug/src/logging.rs.h"
#include "lbug/src/lib.rs.h"

#include "Version.h"

using namespace RE;
using namespace RE::BSScript;
// using namespace SKSE::log;
// using namespace SKSE::stl;
// using namespace SKSE;
// using namespace REL;

#define DllExport __declspec(dllexport)

// Native Events
bool Lb_Process_Event_Bridge(std::monostate, std::string eventName, std::string strArg, float numArg)
{
    return lb_process_event_bridge(eventName, strArg, numArg);
}

constexpr std::string_view PapyrusClass = "Lb_Native";
bool RegisterPapyrusCalls(IVirtualMachine *vm)
{
    vm->BindNativeMethod(PapyrusClass, "Process_Event", Lb_Process_Event_Bridge, false);
    return true;
}

/*

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
    // lb_init_logging(::rust::String(GetLogFile()));

    auto *plugin = PluginDeclaration::GetSingleton();
    auto version = plugin->GetVersion();
    lb_log_info(std::format("{} {} is loading...", plugin->GetName(), version));

    Init(skse);
    InitializePapyrus();
    InitializeMessaging();

    lb_log_info(std::format("{} has finished loading.", plugin->GetName()));
    return true;
}

*/

extern "C" __declspec(dllexport) bool F4SEAPI F4SEPlugin_Query(const F4SE::QueryInterface* a_f4se, F4SE::PluginInfo* a_info)
{
    lb_init_logging(::rust::String("c:\\temp\\experiment.log"));

	a_info->infoVersion = F4SE::PluginInfo::kVersion;
	a_info->name = Version::PROJECT.data();
	a_info->version = Version::MAJOR;

	if (a_f4se->IsEditor()) {
		lb_log_error("loaded in editor");
		return false;
	}

	const auto ver = a_f4se->RuntimeVersion();
	if (ver < F4SE::RUNTIME_1_10_162) {
		lb_log_error(std::format("unsupported runtime v{}", ver.string()));
		return false;
	}

	return true;
}

extern "C" __declspec(dllexport) bool F4SEAPI F4SEPlugin_Load(const F4SE::LoadInterface* a_f4se)
{
	F4SE::Init(a_f4se);
	lb_log_info("plugin loaded");
	return true;
}
