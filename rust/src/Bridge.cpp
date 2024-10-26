#pragma once

#include "rust/cxx.h"
#include "Bridge.h"

// Actor
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
const RE::TESRace* GetRace(const RE::Actor *actor) {
    if (actor == NULL || actor->race == NULL)
    {
        return NULL;
    }
    return actor->race;
}
std::uint32_t GetFormID(const RE::TESForm* form) {
    if (form == NULL)
    {
        return 0;
    }
    return form->GetFormID();
}
const RE::TESForm* AsForm(const RE::TESRace* form) {
    return form;
}

// NiAVObject
const RE::NiAVObject* GetBone(const RE::Actor *actor, rust::Str bone) 
{
    return actor->Get3D()->GetObjectByName( (std::string) bone );   
}
