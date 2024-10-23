#pragma once

#include "rust/cxx.h"
#include "lbug/src/bridge.rs.h"
#include "PCH.h"

// lightweight wrapper on vector<Actor*> cause cxx crate does 
// not allow passing pointers to opaque c++ types in vectors
class ActorVec {
    std::vector<RE::Actor*> actors;
    public: 
        ActorVec(std::vector<RE::Actor*> actors);
        const RE::Actor* GetActor(int pos) const;
        int Size() const;
};

bool IsPlayer(const RE::Actor *actor);
Sex GetSex(const RE::Actor *actor);
const RE::TESRace* GetRace(const RE::Actor *actor);
std::uint32_t GetFormID(const RE::TESForm* form);
const RE::TESForm* AsForm(const RE::TESRace* form);