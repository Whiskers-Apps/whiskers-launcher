import { invoke } from "@tauri-apps/api";
import { appWindow } from '@tauri-apps/api/window';
import { emit } from "@tauri-apps/api/event";

export class ViewModel {
    hasLoaded = false;
    whitelistedApps: WhitelistApp[] = [];
    currentWhitelistedApps: WhitelistApp[] = [];
    checkedCount = 0;

    async load() {
        this.whitelistedApps = await invoke("get_whitelisted_apps");
        this.currentWhitelistedApps = this.whitelistedApps;
        this.hasLoaded = true;
    }

    toggleApp(app: WhitelistApp) {
        const newList: WhitelistApp[] = [];
        let newCheckedCount = 0;

        this.whitelistedApps.forEach((wlApp) => {
            if (wlApp.exec_path === app.exec_path) {
                const newApp = wlApp;
                newApp.checked = !newApp.checked;

                newList.push(newApp);
                if (wlApp.checked) {
                    newCheckedCount += 1;
                }
            } else {
                newList.push(wlApp);

                if (wlApp.checked) {
                    newCheckedCount += 1;
                }
            }
        });

        this.currentWhitelistedApps = newList;
        this.checkedCount = newCheckedCount;
    }

    async addToBlacklist() {
        const checkedPaths: string[] = [];

        this.currentWhitelistedApps.forEach((app) => {
            if (app.checked) {
                checkedPaths.push(app.exec_path);
            }
        });

        await invoke("add_to_blacklist", {paths: checkedPaths});
        emit("load-settings");
        emit("refresh-blacklist");
        appWindow.close();
    }
}

export interface WhitelistApp {
    icon_path: string | undefined;
    name: string;
    exec_path: string;
    checked: boolean;
}
