$ModDeployPath = "C:\Wabbajack\Downloads\paradise2\mods\Telekinesis2" 

# Base Mod
Copy-Item -v    "deploy\01 DLL_163\F4SE\Plugins\*.dll" "$ModDeployPath\F4SE\Plugins\" -Force
Copy-Item -v -r "deploy\00 Default\Scripts\*.pex" "$ModDeployPath\Scripts\" -Force
Copy-Item -v -r "deploy\00 Default\Scripts\TK2\*.pex" "$ModDeployPath\Scripts\TK2\" -Force
Copy-Item -v -r "deploy\00 Default\Scripts\Source\User\TK2\*.psc" "$ModDeployPath\Scripts\Source\User\TK2\" -Force
Copy-Item -v -r "deploy\00 Default\MCM\Config\Telekinesis\config.json" "$ModDeployPath\MCM\Config\Telekinesis\config.json" -Force
Copy-Item -v -r "deploy\00 Default\MCM\Config\Telekinesis\settings.ini" "$ModDeployPath\MCM\Config\Telekinesis\settings.ini" -Force

# Default Configs
Copy-Item -v    "deploy\00 Default\F4SE\Plugins\Telekinesis2\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\" -Force
Copy-Item -v -r "deploy\00 Default\F4SE\Plugins\Telekinesis2\Triggers\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Triggers\" -Force
Copy-Item -v -r "deploy\00 Default\F4SE\Plugins\Telekinesis2\Actions\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Actions\" -Force
Copy-Item -v -r "deploy\00 Default\F4SE\Plugins\Telekinesis2\Races\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Races\" -Force
Copy-Item -v -r "deploy\00 Default\F4SE\Plugins\Telekinesis2\Variables\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Variables\" -Force
Copy-Item -v -r "deploy\00 Default\F4SE\Plugins\Telekinesis2\Patterns\*.funscript" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Patterns\" -Force

# Default Scene (Bone Tracking Enabled)
Copy-Item -v -r "deploy\10 BoneTrackingDefaultScene\F4SE\Plugins\Telekinesis2\Triggers\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Triggers\" -Force

# BodyTalk + Fusion Girl
Copy-Item -v -r "deploy\20 FemaleBodyFusionGirls\F4SE\Plugins\Telekinesis2\Races\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Races\" -Force
Copy-Item -v -r "deploy\30 MaleBodyBodyTalk\F4SE\Plugins\Telekinesis2\Races\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Races\" -Force
