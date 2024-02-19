$localDir = $env:APPDATA + "\whiskers-launcher"
$shortcut = $env:ALLUSERSPROFILE + "\Microsoft\Windows\Start Menu\Programs\Whiskers Launcher.lnk"
$autoStartShortcut = $env:APPDATA + "\Microsoft\Windows\Start Menu\Programs\Startup\Whiskers-Launcher-Companion.lnk"

if(Test-Path -Path $localDir){Remove-Item $localDir}
if(Test-Path -Path $shortcut){Remove-Item $shortcut}
if(Test-Path -Path $autoStartShortcut){Remove-Item $autoStartShortcut}
