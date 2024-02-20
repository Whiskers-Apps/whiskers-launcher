$installationFilesDir = "%installation_files_dir%"
$localDir = $env:APPDATA + "\whiskers-launcher"
$appsDir = $env:ALLUSERSPROFILE + "\Microsoft\Windows\Start Menu\Programs"

$launcherExe = $installationFilesDir + "\whiskers-launcher.exe"
$companionExe = $installationFilesDir + "\whiskers-launcher-companion.exe"
$shortcut = $installationFilesDir + "\Whiskers Launcher.lnk"
$resourcesDir = $installationFilesDir + "\resources"

if(!(Test-Path -Path $localDir)){
    New-Item -ItemType Directory -Path $localDir
}

Copy-Item $launcherExe -Destination $localDir -Force
Copy-Item $companionExe -Destination $localDir -Force
Copy-Item $shortcut -Destination $appsDir -Force
Copy-Item $resourcesDir -Destination $localDir -Force -Recurse