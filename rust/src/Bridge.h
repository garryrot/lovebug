#pragma once
#include "lbug/src/lib.rs.h"
#include "rust/cxx.h"
#include "PCH.h"

// struct ModEvent;
RE::TESForm* GetFormById(int form_id, rust::Str esp) noexcept;