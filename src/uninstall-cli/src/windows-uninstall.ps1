$localDir = $env:APPDATA + "\com-whiskersapps-launcher"
$shortcut = $env:ALLUSERSPROFILE + "\Microsoft\Windows\Start Menu\Programs\Whiskers Launcher.lnk"
$autoStartShortcut = $env:APPDATA + "\Microsoft\Windows\Start Menu\Programs\Startup\Whiskers-Launcher.lnk"

if(Test-Path -Path $localDir){Remove-Item -Recurse -Force $localDir}
if(Test-Path -Path $shortcut){Remove-Item $shortcut}
if(Test-Path -Path $autoStartShortcut){Remove-Item $autoStartShortcut}
