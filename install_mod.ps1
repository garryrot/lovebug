$ModDeployPath = "c:\Mods"

Copy-Item -v    "deploy\Data\F4SE\Plugins\*.dll" "$ModDeployPath\F4SE\Plugins\" -Force
Copy-Item -v    "deploy\Data\F4SE\Plugins\Telekinesis2\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\" -Force
Copy-Item -v -r "deploy\Data\F4SE\Plugins\Telekinesis2\Triggers\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Triggers\" -Force
Copy-Item -v -r "deploy\Data\F4SE\Plugins\Telekinesis2\Actions\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Actions\" -Force
Copy-Item -v -r "deploy\Data\F4SE\Plugins\Telekinesis2\Races\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Races\" -Force
Copy-Item -v -r "deploy\Data\F4SE\Plugins\Telekinesis2\Variables\*.json" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Variables\" -Force
Copy-Item -v -r "deploy\Data\F4SE\Plugins\Telekinesis2\Patterns\*.funscript" "$ModDeployPath\F4SE\Plugins\Telekinesis2\Patterns\" -Force
Copy-Item -v -r "deploy\Data\Scripts\*.pex" "$ModDeployPath\Scripts\" -Force
Copy-Item -v -r "deploy\Data\Scripts\TK2\*.pex" "$ModDeployPath\Scripts\TK2\" -Force
Copy-Item -v -r "deploy\Data\Scripts\Source\User\TK2\*.psc" "$ModDeployPath\Scripts\Source\User\TK2\" -Force
Copy-Item -v -r "deploy\Data\MCM\Config\Telekinesis\config.json" "$ModDeployPath\MCM\Config\Telekinesis\config.json" -Force
Copy-Item -v -r "deploy\Data\MCM\Config\Telekinesis\settings.ini" "$ModDeployPath\MCM\Config\Telekinesis\settings.ini" -Force
