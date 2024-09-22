Scriptname LB_Native extends ScriptObject Native

Bool Function Process_Event(String eventName, String strArg, float floatArg) Native Global
Int Function Action(String actionName, Int speed, Float secs) Native Global
Int Function Scene(String sceneName, String[] tags, Int speed, Float secs) Native Global
Bool Function Update(Int handle, Int speed) Native Global
Bool Function Stop(Int handle) Native Global