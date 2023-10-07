$shortcutPath = $env:APPDATA + "\Microsoft\Windows\Start Menu\Programs\Startup\Simple-KL-Service.lnk"

if(Test-Path -Path $shortcutPath){
    Remove-Item -Path $shortcutPath -Force
}