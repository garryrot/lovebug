Set-Location deploy\Data\Scripts\Source\User
$fallout_path = "C:\Program Files (x86)\Steam\steamapps\common\Fallout 4"
$pyro_path = "$env:USERPROFILE\.vscode\extensions\joelday.papyrus-lang-vscode-3.2.0\pyro"
& $pyro_path/pyro.exe --input-path fallout4.ppj --game-path $fallout_path
Set-Location ../../../../../