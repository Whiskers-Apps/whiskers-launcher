$shortcutPath = $env:APPDATA + "\Microsoft\Windows\Start Menu\Programs\Startup\Whiskers-Launcher.lnk"

Remove-Item -Path $shortcutPath -Force
