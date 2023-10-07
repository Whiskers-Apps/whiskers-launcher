Add-Type -AssemblyName System.Drawing

class App {
    [string] $icon_path
    [string] $exec_path
    [string] $name
}

$TEMP_DIR = $env:TEMP + "\simple-kl"
$SHORTCUTS_DIR = $TEMP_DIR + "\shortcuts"
$ICONS_DIR = $TEMP_DIR + "\icons"
$APPS_YAML_PATH = $TEMP_DIR + "\apps.yml"

$shell = New-Object -ComObject WScript.Shell
$startApps = Get-StartApps
$apps = @()
$appIndex = 0
$appsYaml = ""

#############################################
## Init Directories
#############################################
if(!(Test-Path -Path $SHORTCUTS_DIR)){
    New-Item -ItemType Directory -Path $SHORTCUTS_DIR
}

if(!(Test-Path -Path $ICONS_DIR)){
    New-Item -ItemType Directory -Path $ICONS_DIR
}

#############################################
## Gets Apps
#############################################
foreach($startApp in $startApps){

    $shortcutPath = $SHORTCUTS_DIR + "\$($appIndex).lnk"
    $iconPath = $ICONS_DIR + "\$($appIndex).png" 
    
    #Creates a shortcut
    $shortcut = $shell.CreateShortcut($shortcutPath)
    $shortcut.TargetPath = "shell:AppsFolder\" + $startApp.AppID
    $shortcut.Save()

    #Gets icon from shortcut
    $icon = [System.Drawing.Icon]::ExtractAssociatedIcon($shortcutPath)
    $icon.ToBitmap().Save($iconPath, "png")
    
    #Adds app to apps
    $app = [App]::new()
    $app.name =  $startApp.Name
    $app.exec_path = $shortcutPath
    $app.icon_path = $iconPath

    $apps += $app
    $appIndex += 1
}

#############################################
## Writes Apps Yaml
#############################################

foreach ($app in $apps) {

    $appsYaml += "- icon_path : $($app.icon_path)`n"
    $appsYaml += "  exec_path : $($app.exec_path)`n"
    $appsYaml += "  name : $($app.name)`n"
}

if (!(Test-Path -Path $APPS_YAML_PATH)) {
    New-Item -ItemType File -Path $APPS_YAML_PATH
}

[System.IO.File]::WriteAllLines($APPS_YAML_PATH, $appsYaml)
#PS: It's very important to save as UTF8 AAAAAAAAAAAAAAAAAAAAAAAAAAAA
