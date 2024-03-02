$installationFilesDir = "%installation_files_dir%"
$localDir = $env:APPDATA + "\com-lighttigerxiv-whiskers-launcher"
$appsDir = $env:ALLUSERSPROFILE + "\Microsoft\Windows\Start Menu\Programs"

$launcherExe = $installationFilesDir + "\whiskers-launcher.exe"
$companionExe = $installationFilesDir + "\whiskers-launcher-companion.exe"
$resourcesDir = $installationFilesDir + "\AppResources"

if(!(Test-Path -Path $localDir)){
    New-Item -ItemType Directory -Path $localDir
}

Copy-Item $launcherExe -Destination $localDir -Force
Copy-Item $companionExe -Destination $localDir -Force
Copy-Item $resourcesDir -Destination $localDir -Force -Recurse

$shell = New-Object -ComObject WScript.Shell
$shortcut = $shell.CreateShortcut($appsDir + "\Whiskers Launcher.lnk")
$shortcut.TargetPath = $localDir + "\whiskers-launcher-companion.exe"
$shortcut.Save()