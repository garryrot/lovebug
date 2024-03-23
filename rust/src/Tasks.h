#pragma once
#include "lbug/src/lib.rs.h"
#include "rust/cxx.h"

struct SKSEModEvent;

void shim_TaskInterface_AddTask(
     rust::Fn<void(SKSEModEvent, rust::Str)> done, 
     SKSEModEvent ctx,
     rust::Str event_name) noexcept;
