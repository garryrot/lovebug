#pragma once

#include "rust/cxx.h"
#include "PCH.h"

float GetDistance(
    RE::NiAVObject* boneA,
    RE::NiAVObject* boneB);

RE::NiAVObject* GetBoneFromActor(const RE::Actor *actor, rust::Str bone);
