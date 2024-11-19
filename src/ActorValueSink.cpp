
class ActorValueSink : public RE::BSTEventSink<ActorValueEvents::ActorValueChangedEvent>
{
public:
    static ActorValueSink *GetSingleton()
    {
        static ActorValueSink singleton;
        return &singleton;
    }

    virtual RE::BSEventNotifyControl ProcessEvent(
        const ActorValueEvents::ActorValueChangedEvent& event, 
        RE::BSTEventSource<ActorValueEvents::ActorValueChangedEvent>*) override
	{
        auto formId = event.actorValue.GetFormID();
        auto value = RE::PlayerCharacter::GetSingleton()->GetActorValue(event.actorValue);
        lb_actor_value_changed(formId, value);
        
        return RE::BSEventNotifyControl::kContinue;
	}
};
