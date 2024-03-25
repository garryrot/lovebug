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

bool Init(SFT) { 
}
 
constexpr std::string_view PapyrusClass = "Lovebug";
bool RegisterPapyrusCalls(IVirtualMachine* vm) {
    // vm->RegisterFunction("Init", PapyrusClass, init);
    return true;
}

void InitializeMessaging() {
    if (!GetMessagingInterface()->RegisterListener([](MessagingInterface::Message* message) {
            switch (message->type) {
                case MessagingInterface::kDataLoaded:
                    // All ESM/ESL/ESP plugins are loaded, forms can be used
                    lb_init();
                    break;
            }
        })) {
        lb_log_info("Failed registering message interface");
    }
}

void InitializePapyrus() {
    lb_log_debug("Initializing Papyrus binding...");
    if (GetPapyrusInterface()->Register(RegisterPapyrusCalls)) {
        lb_log_debug("Papyrus functions bound.");
    } else {
        lb_log_error("Failed binding papyrus");
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
