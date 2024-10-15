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