#include <stddef.h>
#include <thread>
#include <chrono>
#include <codecvt>

#include "Bridge.h"

#include "tk2/src/lib.rs.h"
#include "tk2/src/mcm.rs.h"
#include "tk2/src/logging.rs.h"
// #include "lb/src/lib.rs.h"

#include "Version.h"

using namespace RE;
using namespace RE::BSScript;

#include "Logs.cpp"
#include "MCM.cpp"
#include "Native.cpp"
#include "Events.cpp"
#include "ActorValueSink.cpp"

void InitializeMessaging()
{
    const auto messaging = F4SE::GetMessagingInterface();
    if (!messaging || !messaging->RegisterListener([](F4SE::MessagingInterface::Message *message)
        {
            switch (message->type) {
                case F4SE::MessagingInterface::kGameDataReady: {
                    lb_log_info("game data ready");
                    GameVM* gameVm = RE::GameVM::GetSingleton();
                    if (gameVm) {
                        gameVm->GetVM()->RegisterForLogEvent(LogEventSink::GetSingleton());
                    }
                    break;
                }
                case F4SE::MessagingInterface::kGameLoaded: {
                    lb_log_info("game loaded");
                    auto playerValueSink = ActorValueSink::GetSingleton();
                    RE::TESObjectREFR *player = RE::PlayerCharacter::GetSingleton();
                    player->RegisterSink(playerValueSink);
                    break;
                }
            }
        })) {
        lb_log_error("Failed to get messaging interface");
        return;
    } else {
        lb_log_info("Registered messaging interface");
    }
}

#ifdef F4SEPluginVersion
F4SE_EXPORT constinit auto F4SEPlugin_Version = []() noexcept {
	F4SE::PluginVersionData data{};
	data.PluginName(Version::PROJECT.data());
	data.PluginVersion({
		Version::MAJOR,
		Version::MINOR,
		Version::PATCH,
    });
	data.AuthorName("garryrot");
	data.UsesAddressLibrary(true);
	data.UsesSigScanning(false);
	data.IsLayoutDependent(true);
	data.HasNoStructUse(false);
	data.CompatibleVersions({ F4SE::RUNTIME_LATEST }); // F4SE::RUNTIME_LATEST_VR might work but I don't know
	return data;
}();
#endif

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
	if (ver < F4SE::RUNTIME_LATEST) {
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
