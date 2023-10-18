$installationFilesDir = "%installation_files_dir%"
$localDir = $env:APPDATA + "\simple-kl"
$appsDir = $env:ALLUSERSPROFILE + "\Microsoft\Windows\Start Menu\Programs"

$launcherExe = $installationFilesDir + "\simple-keyboard-launcher.exe"
$serviceExe = $installationFilesDir + "\simple-kl-service.exe"
$shortcut = $installationFilesDir + "\Simple Keyboard Launcher.lnk"
$resourcesDir = $installationFilesDir + "\resources"

if(!(Test-Path -Path $localDir)){
    New-Item -ItemType Directory -Path $localDir
}

Copy-Item $launcherExe -Destination $localDir -Force
Copy-Item $serviceExe -Destination $localDir -Force
Copy-Item $shortcut -Destination $appsDir -Force
Copy-Item $resourcesDir -Destination $localDir -Force -Recurse