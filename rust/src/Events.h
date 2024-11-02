#pragma once
#include "Bridge.h"
#include "tk2/src/events.rs.h"

void AddTask_ModEvent(rust::Fn<void(ModEvent)> done, ModEvent ctx) noexcept;
void SendEvent(ModEvent event) noexcept;
