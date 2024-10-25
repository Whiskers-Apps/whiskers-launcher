import { appWindow } from "@tauri-apps/api/window";

export function onCloseDialog(){
    appWindow.close();
}