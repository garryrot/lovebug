#pragma once
#include "Events.h"
#include "F4SE/API.h"
#include "RE/Bethesda/BSTArray.h"
#include "RE/Bethesda/BSScript.h"

// Outgoing Events

void AddTask_SKSEModEvent(rust::Fn<void(SKSEModEvent)> done, SKSEModEvent ctx) noexcept {
    std::function<void()> task = std::move([done, ctx]() mutable {
        (*done)(ctx);
    });
    F4SE::GetTaskInterface()->AddTask(task);
}

void SendEvent(RE::TESForm* form, SKSEModEvent event) noexcept {
    if (form == NULL) {
        return;
    }
    // TODO: Send the event
}
