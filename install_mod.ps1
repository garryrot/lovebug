$ModDeployPath = "mods\Telekinesis"

Copy-Item -v    "deploy\Data\F4SE\Plugins\*.dll" "$ModDeployPath\F4SE\Plugins\"
Copy-Item -v -r "deploy\Data\F4SE\Plugins\Lovebug\Triggers\*.json" "$ModDeployPath\F4SE\Plugins\Lovebug\Triggers\"
Copy-Item -v -r "deploy\Data\F4SE\Plugins\Lovebug\Actions\*.json" "$ModDeployPath\F4SE\Plugins\Lovebug\Actions\"
Copy-Item -v -r "deploy\Data\F4SE\Plugins\Lovebug\Patterns\*.funscript" "$ModDeployPath\F4SE\Plugins\Lovebug\Patterns\"
Copy-Item -v -r "deploy\Data\Scripts\*.pex" "$ModDeployPath\Scripts"
Copy-Item -v -r "deploy\Data\Scripts\Source\User\*.psc" "$ModDeployPath\Scripts\Source\User\"
