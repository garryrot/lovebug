Scriptname Lb_Main extends Quest  

Event OnInit()
    Debug.MessageBox("OnInit")
    RegisterForModEvent("event_foo", "OnFoo")
    RegisterForModEvent("event_bar", "OnBar")
EndEvent

Event OnFoo(String eventName, String strArg, Float numArg, Form sender)
    Debug.MessageBox(eventName)
EndEvent

Event OnBar(String eventName, String strArg, Float numArg, Form sender)
    Debug.MessageBox(eventName)
EndEvent