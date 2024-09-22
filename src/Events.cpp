
class LogEventSink : public RE::BSTEventSink<LogEvent>
{
public:
    static LogEventSink *GetSingleton()
    {
        static LogEventSink singleton;
        return &singleton;
    }

    virtual RE::BSEventNotifyControl ProcessEvent(const LogEvent& event, RE::BSTEventSource<LogEvent>*) override
	{
        BSFixedString temp = NULL;
        event.errorMsg.GetErrorMsg(temp);
        std::string message = (std::string) temp;

        if (message.find("plug start to vibrate") != std::string::npos) {
            lb_log_info("FOUND MATCH");
        }
        
        return RE::BSEventNotifyControl::kContinue;
	}
};
