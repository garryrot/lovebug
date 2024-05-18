#pragma once
#include "rust/cxx.h"
#include "Bridge.h"

// Wrap TESForm

RE::TESForm* GetFormById(int form_id, rust::Str esp) noexcept {
    return NULL;// RE::TESDataHandler::GetSingleton()->LookupForm<RE::TESQuest>(form_id, (std::string) esp);
}

/*
RE::TESForm* GetSender(const SKSE::ModCallbackEvent* event) noexcept {
    if (event != NULL) {
        return event->sender;
    }
    return NULL;
}
*/