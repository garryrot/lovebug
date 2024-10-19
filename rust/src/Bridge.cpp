#pragma once

#include "rust/cxx.h"
#include "Bridge.h"

ActorVec::ActorVec(std::vector<RE::Actor*> actors) {
    this->actors = actors;
}

const RE::Actor* ActorVec::GetActor( int pos ) const {
    return this->actors[ pos ];
}

int ActorVec::Size() const {
    return this->actors.size();
}

bool IsPlayer(const RE::Actor *actor) {
    return actor == RE::PlayerCharacter::GetSingleton();
}

Sex GetSex(const RE::Actor *actor) {
    auto npc = actor->GetNPC();
    if (npc != NULL)
    {
        if (npc->GetSex() == 1) {
            return Sex::Female;
        } else {
            return Sex::Male;
        }
    }
    return Sex::None;
}