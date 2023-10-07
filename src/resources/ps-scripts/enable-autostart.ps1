$shell = New-Object -ComObject WScript.Shell

$autoStartDir = $env:APPDATA + "\Microsoft\Windows\Start Menu\Programs\Startup"
$service_path = $env:APPDATA + "\simple-kl\simple-kl-service.exe"
$shortcut_path = $autoStartDir + "\Simple-KL-Service.lnk"

$shortcut = $shell.CreateShortcut($shortcut_path)
$shortcut.TargetPath = $service_path
$shortcut.Save()
