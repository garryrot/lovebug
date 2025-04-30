
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
        if (lb_is_loaded())
        {
            auto formId = event.actorValue.GetFormID();
            auto value = RE::PlayerCharacter::GetSingleton()->GetActorValue(event.actorValue);
            lb_process_actor_value(formId, value);
        }
        
        return RE::BSEventNotifyControl::kContinue;
	}
};
