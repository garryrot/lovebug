#pragma once

#include "rust/cxx.h"
#include "Bridge.h"

// Actor
const RE::Actor* PlayerCharacter_GetSingleton() {
    return RE::PlayerCharacter::GetSingleton();
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

bool ContainsKeyword(const RE::Actor *actor, rust::Str editorId) {
    auto isVibratingKeyword = RE::TESForm::GetFormByEditorID( (std::string) editorId);
    if (isVibratingKeyword && isVibratingKeyword->GetSavedFormType() == RE::ENUM_FORM_ID::kKYWD)
    {
        
        return actor->HasKeyword( (const RE::BGSKeyword *) isVibratingKeyword, nullptr);
    }
    return false;
}

// float GetPlayerActorValue(rust::Str editorId) {
//     std::string actorValueEditorId = (std::string) editorId;
//     auto form = RE::TESForm::GetFormByEditorID(actorValueEditorId);
//     if (form)
//     {
//         auto player = RE::PlayerCharacter::GetSingleton();

//         RE::ENUM_FORM_ID formType = form->GetSavedFormType();
//         if (formType == RE::ENUM_FORM_ID::kAVIF)
//         {
//             auto a_info = (RE::ActorValueInfo*) form;
//             float actorValue = player->GetActorValue( (*a_info) );
//             lb_log_info(std::format("Read actor value {} from formId={:x}: {}", actorValueEditorId, form->formID, actorValue));
//             return actorValue;
//         }
//         else 
//         {
//             lb_log_error(
//                 std::format("Editor id {} with formID={:x} is not actor value, form type: {:x}", 
//                 actorValueEditorId, 
//                 form->formID, 
//                 static_cast<unsigned>(formType)));
//         }
//     }
//     return 0;
// }

// ActorVec
ActorVec::ActorVec(std::vector<RE::Actor*> actors) {
    this->actors = actors;
}
const RE::Actor* ActorVec::GetActor( int pos ) const {
    return this->actors[ pos ];
}
int ActorVec::Size() const {
    return this->actors.size();
}

// Form
const RE::TESForm* AsForm(const RE::TESRace* form) {
    return form;
}
const RE::TESForm* TESForm_GetFormByEditorID(rust::Str editorId) {
    std::string editorIdStr = (std::string) editorId;
    return RE::TESForm::GetFormByEditorID( editorIdStr );
}
std::uint32_t GetFormID(const RE::TESForm* form) {
    if (form == NULL)
    {
        return 0;
    }
    return form->GetFormID();
}
std::uint32_t GetSavedFormType(const RE::TESForm* form) {
    if (form == NULL)
    {
        return 0;
    }
    return static_cast<std::uint32_t>(form->GetSavedFormType());
}

// NiAVObject
const RE::NiAVObject* GetBone(const RE::Actor *actor, rust::Str bone) 
{
    return actor->Get3D()->GetObjectByName( (std::string) bone );   
}
