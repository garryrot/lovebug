#pragma once

#include "rust/cxx.h"
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
