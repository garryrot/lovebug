#pragma once

#include "rust/cxx.h"
#include "lbug/src/bones.rs.h"
#include "PCH.h"

float GetDistance(
    RE::NiAVObject* boneA,
    RE::NiAVObject* boneB);

RE::NiAVObject* GetBoneFromActor(const RE::Actor *actor, rust::Str bone);

bool IsPlayer(const RE::Actor *actor);
Sex GetSex(const RE::Actor *actor);
