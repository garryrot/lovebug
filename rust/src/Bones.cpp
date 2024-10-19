
RE::NiAVObject* GetBoneFromActor(const RE::Actor *actor, rust::Str bone) 
{
    return actor->Get3D()->GetObjectByName( (std::string) bone );   
}

float GetDistance(
    RE::NiAVObject* boneA,
    RE::NiAVObject* boneB) {
    if (boneA == NULL)
    {
        return 999999.0;
    }
    if (boneB == NULL)
    {
        return 999999.0;
    }
    return sqrtf(powf((boneA->world.translate.x - boneB->world.translate.x), 2) +
                 powf((boneA->world.translate.y - boneB->world.translate.y), 2) +
                 powf((boneA->world.translate.z - boneB->world.translate.z), 2));
}
