#include <stddef.h>

#include <thread>
#include <chrono>
#include <codecvt>

#include "lbug/src/logging.rs.h"
#include "lbug/src/lib.rs.h"

#include "Version.h"

using namespace RE;
using namespace RE::BSScript;

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
*/

std::string GetLogFile() {
	auto path = F4SE::log::log_directory();
    if (!path) {
        return "";
    }
    std::optional<std::string> utf8Path = unicode_to_utf8(path->wstring());
    if (!utf8Path.has_value()) {
        return "";
    }
    return std::format("{}\\{}.log", utf8Path.value(), Version::PROJECT.data());
}

/*
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

std::string unicode_to_utf8(std::wstring in) {
    using convert_type = std::codecvt_utf8<wchar_t>;
    std::wstring_convert<convert_type, wchar_t> converter;
    std::string converted_str = converter.to_bytes( in );
    return converted_str;
}

// TODO: Port to SSE

extern "C" __declspec(dllexport) bool F4SEAPI F4SEPlugin_Query(const F4SE::QueryInterface* a_f4se, F4SE::PluginInfo* a_info)
{
    lb_init_logging(GetLogFile());

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
