$localDir = $env:APPDATA + "\simple-kl"
$shortcut = $env:ALLUSERSPROFILE + "\Microsoft\Windows\Start Menu\Programs\Simple Keyboard Launcher.lnk"
$autoStartShortcut = $env:APPDATA + "\Microsoft\Windows\Start Menu\Programs\Startup\Simple-KL-Service.lnk"

if(Test-Path -Path $localDir){Remove-Item $localDir}
if(Test-Path -Path $shortcut){Remove-Item $shortcut}
if(Test-Path -Path $autoStartShortcut){Remove-Item $autoStartShortcut}
