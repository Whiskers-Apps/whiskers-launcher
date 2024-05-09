Add-Type -AssemblyName System.Drawing

class App {
    [string] $id
    [string] $icon_path
    [string] $exec_path
    [string] $name
}

$INDEXING_DIR = $env:APPDATA + "\com-lighttigerxiv-whiskers-launcher\Indexing"
$SHORTCUTS_DIR = $INDEXING_DIR + "\Shortcuts"
$ICONS_DIR = $INDEXING_DIR + "\Icons"
$APPS_JSON_PATH = $INDEXING_DIR + "\apps.json"

$shell = New-Object -ComObject WScript.Shell
$startApps = Get-StartApps
$apps = @()
$appIndex = 0
$appsJson = ""

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
    $app.id = $startApp.AppID
    $app.name =  $startApp.Name
    $app.exec_path = $shortcutPath
    $app.icon_path = $iconPath

    $apps += $app
    $appIndex += 1
}

#############################################
## Writes Apps Json
#############################################

$appsJson += "[`n"
foreach ($app in $apps) {
    $appId = $app.id -replace "\\", "\\"
    $iconPath = $app.icon_path -replace "\\", "\\"
    $execPath = $app.exec_path -replace "\\", "\\"
    
    $appsJson += "{`n"
    $appsJson += "`"id`" : `"$appId`",`n"
    $appsJson += "`"icon_path`" : `"$iconPath`",`n"
    $appsJson += "`"exec_path`" : `"$execPath`",`n"
    $appsJson += "`"name`" : `"$($app.name)`"`n"
    $appsJson += "},`n"
}
$appsJson = $appsJson.Substring(0, $appsJson.Length - 2)
$appsJson += "`n]"

if (!(Test-Path -Path $APPS_JSON_PATH)) {
    New-Item -ItemType File -Path $APPS_JSON_PATH
}

[System.IO.File]::WriteAllLines($APPS_JSON_PATH, $appsJson)
#PS: It's very important to save as UTF8 AAAAAAAAAAAAAAAAAAAAAAAAAAAA
