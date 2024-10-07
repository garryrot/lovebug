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

#include "Logs.cpp"
#include "Bones.cpp"
#include "Native.cpp"
#include "Events.cpp"

// Messaging
void InitializeMessaging()
{
    const auto messaging = F4SE::GetMessagingInterface();
    if (!messaging || !messaging->RegisterListener([](F4SE::MessagingInterface::Message *message)
        {
            switch (message->type) {
                case F4SE::MessagingInterface::kInputLoaded: {
                    lb_log_info("input loaded");
                    break;
                }
                case F4SE::MessagingInterface::kGameDataReady: {
                    lb_log_info("game data ready");
                    GameVM* gameVm = RE::GameVM::GetSingleton();
                    if (gameVm) {
                        gameVm->GetVM()->RegisterForLogEvent(LogEventSink::GetSingleton());
                    }
                    lb_init();
                    break;
                }
                case F4SE::MessagingInterface::kGameLoaded:
                    lb_log_info("game loaded");
                    
                    break;
            }
        })) {
        lb_log_error("Failed to get messaging interface");
        return;
    } else {
        lb_log_info("Registered messaging interface");
    }
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
    InitializeNative();
	return true;
}
