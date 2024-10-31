#pragma once

#include "rust/cxx.h"
#include "tk2/src/bridge.rs.h"
#include "PCH.h"

// Actor
bool IsPlayer(const RE::Actor *actor);
Sex GetSex(const RE::Actor *actor);
const RE::TESRace* GetRace(const RE::Actor *actor);

// lightweight wrapper on vector<Actor*> cause cxx crate does 
// not allow passing pointers to opaque c++ types in vectors
class ActorVec {
    std::vector<RE::Actor*> actors;
    public: 
        ActorVec(std::vector<RE::Actor*> actors);
        const RE::Actor* GetActor(int pos) const;
        int Size() const;
};

std::uint32_t GetFormID(const RE::TESForm* form);
const RE::TESForm* AsForm(const RE::TESRace* form);
const RE::NiAVObject* GetBone(const RE::Actor *actor, rust::Str bone);