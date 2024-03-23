#pragma once
#include "rust/cxx.h"
#include "Tasks.h"

void shim_TaskInterface_AddTask(
        rust::Fn<void(SKSEModEvent, rust::Str)> done, 
        SKSEModEvent ctx, 
        rust::Str event_name
    ) noexcept {
    std::function<void()> func_move = std::move([done, ctx, event_name]() mutable {
        (*done)(ctx, event_name);
    });
    SKSE::GetTaskInterface()->AddTask(func_move);
}