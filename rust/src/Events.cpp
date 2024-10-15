#pragma once
#include "Events.h"
#include "F4SE/API.h"
#include "lbug/src/logging.rs.h"
#include "RE/Bethesda/BSTSmartPointer.h"

// Outgoing Events

void AddTask_ModEvent(rust::Fn<void(ModEvent)> done, ModEvent ctx) noexcept {
    std::function<void()> task = std::move([done, ctx]() mutable {
        (*done)(ctx);
    });

    F4SE::GetTaskInterface()->AddTask(task);
}

void SendEvent(ModEvent event) noexcept {
    F4SE::GetPapyrusInterface()->GetExternalEventRegistrations(
        event.event_name.c_str(), 
        &event, 
        [](uint64_t handle, const char* scriptName, const char* callbackName, void* data
    ) {
		ModEvent* evt = static_cast<ModEvent*>( data );
		std::string strArg = (std::string) evt->str_arg;
		if (auto* vm = RE::GameVM::GetSingleton()) {
			vm->GetVM().get()->DispatchMethodCall(handle, scriptName, callbackName, nullptr, strArg, evt->num_arg);
		}
    });
}
