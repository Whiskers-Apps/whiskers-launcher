$shell = New-Object -ComObject WScript.Shell

$autoStartDir = $env:APPDATA + "\Microsoft\Windows\Start Menu\Programs\Startup"
$companionPath = $env:APPDATA + "\com-whiskersapps-launcher\whiskers-launcher-companion.exe"
$shortcutPath = $autoStartDir + "\Whiskers-Launcher.lnk"

$shortcut = $shell.CreateShortcut($shortcutPath)
$shortcut.TargetPath = $companionPath
$shortcut.Save()
