#pragma once
#include "rust/cxx.h"
#include "Bridge.h"

// void AddTask_SKSEModEvent(rust::Fn<void(SKSEModEvent)> done, SKSEModEvent ctx) noexcept {
//     std::function<void()> task = std::move([done, ctx]() mutable {
//         (*done)(ctx);
//     });
//     SKSE::GetTaskInterface()->AddTask(task);
// }

// void SendEvent(RE::TESForm* form, SKSEModEvent event) noexcept {
//     if (form == NULL) {
//         return;
//     }
//     SKSE::ModCallbackEvent modEvent{ (std::string) event.event_name, (std::string) event.str_arg, (float) event.num_arg, form };
//     auto modCallbackEventSource = SKSE::GetModCallbackEventSource();
//     modCallbackEventSource->SendEvent(&modEvent);
// }

// RE::TESForm* GetFormById(int form_id, rust::Str esp) noexcept {
//     return RE::TESDataHandler::GetSingleton()->LookupForm<RE::TESQuest>(form_id, (std::string) esp);
// }

// SKSEModEvent CloneInto(const SKSE::ModCallbackEvent* event) noexcept {
//     return {(std::string) event->eventName, (std::string) event->strArg, event->numArg};
// }

// RE::TESForm* GetSender(const SKSE::ModCallbackEvent* event) noexcept {
//     if (event != NULL) {
//         return event->sender;
//     }
//     return NULL;
// }