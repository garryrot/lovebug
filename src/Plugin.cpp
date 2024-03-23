#include <stddef.h>

#include <thread>
#include <chrono>
#include <stdlib.h>     //for using the function sleep

#include "lbug/src/logging.rs.h"
#include "lbug/src/lib.rs.h"

using namespace RE;
using namespace RE::BSScript;
using namespace SKSE::log;
using namespace SKSE::stl;
using namespace SKSE;
using namespace REL;

#define DllExport __declspec(dllexport)
#define SFT StaticFunctionTag*

bool EventThreadStarted = false;
std::thread EventThread;
RE::TESQuest* MainQuest = NULL;

/// Rust FFI functions to telekinesis crate

void Event_Thread() {
    while (true) {
        auto list = lb_qry_nxt_evt();
        std::vector<SKSEModEvent> evts;
        std::copy(list.begin(), list.end(), std::back_inserter(evts));
        for (int i = 0; i < list.size(); i++) {
            auto eventName = (std::string)evts[i].event_name;
            auto strArg = (std::string)evts[i].str_arg;
            auto numArg = (float)evts[i].num_arg;
            
            SKSE::GetTaskInterface()->AddTask([eventName, strArg, numArg] {
                SKSE::ModCallbackEvent modEvent{ eventName, strArg, numArg, MainQuest };
                auto modCallbackEventSource = SKSE::GetModCallbackEventSource();
                modCallbackEventSource->SendEvent(&modEvent);
            });
        }
        if (evts.size() == 0) {
            lb_log_info("evt dispatch not ready, sleeping...");
            std::this_thread::sleep_for(5000ms);
        }
    }
}

// void Bone_Monitoring_Prototype() {
//     while (true)
//     {
//         std::this_thread::sleep_for(100ms);
//         RE::Actor* player = RE::PlayerCharacter::GetSingleton();
//         auto bone = player->GetNodeByName("NPC Anus Deep1");
//         if (bone != NULL) {
//             float x = bone->world.translate.x;
//             float y = bone->world.translate.y;
//             float z = bone->world.translate.z;
//             auto str = std::format("[bone] npc anus deep1 x={}, y={} z={}", x, y, z);
//             lb_log_info(str);
//         }
//         else {
//             lb_log_info("bone is null");
//         }
//     } 
// }
 
constexpr std::string_view PapyrusClass = "Lovebug";

bool RegisterPapyrusCalls(IVirtualMachine* vm) {
    // vm->RegisterFunction("Loaded", PapyrusClass, api_loaded);
    return true;
}

void InitializeMessaging() {
    if (!GetMessagingInterface()->RegisterListener([](MessagingInterface::Message* message) {
            switch (message->type) {
                case MessagingInterface::kDataLoaded:
                    // All ESM/ESL/ESP plugins are loaded, forms can be used
                    MainQuest = RE::TESDataHandler::GetSingleton()->LookupForm<RE::TESQuest>(0x12C2, "Telekinesis.esp");
                    if (EventThreadStarted) {
                        EventThread.~thread();
                    }
                    EventThreadStarted = true;
                    EventThread = std::thread(Event_Thread);
                    // BoneThread = std::thread(Bone_Monitoring_Prototype);
                    break;
            }
        })) {
        lb_log_info("Failed registering message interface");
    }
}

void InitializePapyrus() {
    log::trace("Initializing Papyrus binding...");
    if (GetPapyrusInterface()->Register(RegisterPapyrusCalls)) {
        log::debug("Papyrus functions bound.");
    } else {
        stl::report_and_fail("Failure to register Papyrus bindings.");
    }
}

std::string GetLogFile() {
    auto path = log_directory();
    if (!path) {
        report_and_fail("Unable to lookup SKSE logs directory.");
    }
    return std::format("{}\\{}.log", path->string(), PluginDeclaration::GetSingleton()->GetName());
}

SKSEPluginLoad(const LoadInterface* skse) {
    lb_init_logging(::rust::String(GetLogFile())); 

    auto* plugin = PluginDeclaration::GetSingleton();
    auto version = plugin->GetVersion();
    lb_log_info(std::format("{} {} is loading...", plugin->GetName(), version));

    Init(skse);
    InitializePapyrus();
    InitializeMessaging();

    lb_log_info(std::format("{} has finished loading.", plugin->GetName()));
    return true;
}
