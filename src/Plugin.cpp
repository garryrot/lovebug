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

void InitializePapyrus()
{
    lb_log_debug("Initializing Papyrus binding...");
    if (! F4SE::GetPapyrusInterface()->Register(RegisterPapyrusCalls))
    {
        lb_log_error("Failed binding papyrus");
    }
}

// Messaging

void InitializeMessaging()
{
    const auto messaging = F4SE::GetMessagingInterface();
    if (!messaging || !messaging->RegisterListener([](F4SE::MessagingInterface::Message *message)
        {
            switch (message->type) {
                case F4SE::MessagingInterface::kInputLoaded:
                    lb_log_info("input loaded");
                    break;
                case F4SE::MessagingInterface::kGameDataReady:
                    lb_log_info("game loaded");
                    lb_init();
                    break;
            }
        })) {
        lb_log_error("Failed to get messaging interface");
        return;
    } else {
        lb_log_error("Registered messaging interface");
    }
}

// TODO: Port to SSE
std::string unicode_to_utf8(std::wstring in) {
    using convert_type = std::codecvt_utf8<wchar_t>;
    std::wstring_convert<convert_type, wchar_t> converter;
    std::string converted_str = converter.to_bytes( in );
    return converted_str;
}

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

extern "C" __declspec(dllexport) bool F4SEAPI F4SEPlugin_Query(const F4SE::QueryInterface* f4se, F4SE::PluginInfo* info)
{
    lb_init_logging(GetLogFile());

	info->infoVersion = F4SE::PluginInfo::kVersion;
	info->name = Version::PROJECT.data();
	info->version = Version::MAJOR;
    lb_log_info(std::format("{} {} is loading...", info->name, info->version));
	if (f4se->IsEditor()) {
		lb_log_error("loaded in editor");
		return false;
	}

	const auto ver = f4se->RuntimeVersion();
	if (ver < F4SE::RUNTIME_1_10_162) {
		lb_log_error(std::format("unsupported runtime v{}", ver.string()));
		return false;
	}

	return true;
}

extern "C" __declspec(dllexport) bool F4SEAPI F4SEPlugin_Load(const F4SE::LoadInterface* f4se)
{
	F4SE::Init(f4se);
	lb_log_info("plugin loaded");
    InitializeMessaging();
    InitializePapyrus();
	return true;
}
