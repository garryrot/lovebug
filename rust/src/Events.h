#pragma once
#include "Bridge.h"
#include "lbug/src/events.rs.h"

void AddTask_ModEvent(rust::Fn<void(ModEvent)> done, ModEvent ctx) noexcept;
void SendEvent(RE::TESForm* form, ModEvent event) noexcept;