#pragma once
#include "Bridge.h"
#include "lbug/src/events.rs.h"

void AddTask_SKSEModEvent(rust::Fn<void(SKSEModEvent)> done, SKSEModEvent ctx) noexcept;
void SendEvent(RE::TESForm* form, SKSEModEvent event) noexcept;