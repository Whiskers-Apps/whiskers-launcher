$shortcutPath = $env:APPDATA + "\Microsoft\Windows\Start Menu\Programs\Startup\Whiskers-Launcher.lnk"

if(Test-Path -Path $shortcutPath){
    Remove-Item -Path $shortcutPath -Force
}