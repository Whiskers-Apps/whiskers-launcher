<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Settings, getSettings, ExtensionSettings as SettingsExtensionsSettings, getTheme } from './Settings';
import { invoke } from '@tauri-apps/api';
import { ExtensionManifest, ExtensionSettings } from '../../data';
import ChevronDownSVG from "../../assets/icons/chevron-down.svg"
import TrashSVG from "../../assets/icons/trash.svg"
import ThreeDotsSVG from "../../assets/icons/three_dots_vertical.svg"
import { listen } from '@tauri-apps/api/event';
import { WebviewWindow } from '@tauri-apps/api/window';
import RestoreButton from '../../components/RestoreButton.vue';

const settings = ref<Settings>();
const tabExtensions = ref<TabExtensionManifest[]>();
const os = ref("");

const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");
const dangerColor = ref("");
const onDangerColor = ref("");
const textColor = ref("");
const secondaryTextColor = ref("");

const updateThemeEmit = ref();

const menu = ref();
const updateExtensionsEmit = ref();


onMounted(async () => {
    settings.value = await getSettings();
    os.value = await invoke("get_os");

    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    });

    getExtensions();

    updateExtensionsEmit.value = listen("updateExtensions", (_event) => {
        getExtensions();
    });
})

async function loadTheme() {
    let theme = await getTheme();
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    accentColor.value = theme.accent;
    dangerColor.value = theme.danger;
    onDangerColor.value = theme.on_danger;
    textColor.value = theme.text;
    secondaryTextColor.value = theme.secondary_text;
}

export interface TabExtensionManifest {
    id: string,
    version: string,
    name: string,
    description?: string,
    icon: string,
    os: string,
    keyword: string,
    current_keyword: string,
    settings?: ExtensionSettings,
    current_settings: SettingsExtensionsSettings[]
}

async function updateSetting(extensionID: string, settingID: string, newValue: string) {
    invoke("update_extension_setting", { extension_id: extensionID, setting_id: settingID, new_value: newValue })
    settings.value = await getSettings();
}

function canShowSetting(extensionID: string, settingID: string): boolean {

    let showSetting = false

    tabExtensions.value?.forEach(extension => {
        extension.settings?.any?.forEach(setting => {
            if (setting.id == settingID) {
                if (setting.show_condition === null) {
                    showSetting = true
                } else {
                    setting.show_condition?.forEach(condition => {

                        if (getSettingValue(extensionID, condition.setting) === condition.value) {
                            showSetting = true
                        }
                    })
                }
            }
        });

        extension.settings?.linux?.forEach(setting => {
            if (setting.id == settingID) {
                if (setting.show_condition === null) {
                    showSetting = true
                } else {
                    setting.show_condition?.forEach(condition => {

                        if (getSettingValue(extensionID, condition.setting) === condition.value) {
                            showSetting = true
                        }
                    })
                }
            }
        });

        extension.settings?.windows?.forEach(setting => {
            if (setting.id == settingID) {
                if (setting.show_condition === null) {
                    showSetting = true
                } else {
                    setting.show_condition?.forEach(condition => {

                        if (getSettingValue(extensionID, condition.setting) === condition.value) {
                            showSetting = true
                        }
                    })
                }
            }
        })
    })

    return showSetting
}

function getSettingValue(extensionID: string, settingID: string): String {

    let settingValue = ""

    settings.value?.extensions.forEach((extension) => {
        if (extension.id == extensionID) {
            extension.settings?.any?.forEach((setting) => {
                if (setting.id == settingID) {
                    settingValue = setting.current_value;
                }
            })
            extension.settings?.linux?.forEach((setting) => {
                if (setting.id == settingID) {
                    settingValue = setting.current_value;
                }
            })
            extension.settings?.windows?.forEach((setting) => {
                if (setting.id == settingID) {
                    settingValue = setting.current_value;
                }
            })
        }
    })

    return settingValue
}

function getExtensionKeyword(extensionID: string): string {

    if (settings.value?.extensions?.find(extension => extension.id === extensionID)?.keyword !== undefined) {
        return settings.value!!.extensions.find(extension => extension.id === extensionID)!!.keyword
    } else {
        return ""
    };

}

async function updateExtensionKeyword(extensionID: string, keyword: string) {

    if (keyword.trim() === "") { return }

    invoke("update_extension_keyword", { extension_id: extensionID, keyword: keyword });
    settings.value = await getSettings();
}

async function getExtensions() {

    settings.value = await getSettings();
    let extensions: ExtensionManifest[] = JSON.parse(await invoke("get_extensions_json") ?? []);
    let newTabExtensions: TabExtensionManifest[] = [];



    extensions.forEach(extension => {

        let newTabExtension: TabExtensionManifest = {
            id: extension.id,
            version: extension.version,
            name: extension.name,
            description: extension.description,
            icon: extension.icon,
            os: extension.os,
            keyword: extension.keyword,
            current_keyword: getExtensionKeyword(extension.id),
            settings: extension.settings,
            current_settings: settings.value!!.extensions
        }

        newTabExtensions.push(newTabExtension);
    });

    tabExtensions.value = newTabExtensions;
}

function toggleIconMenu() {

    if (menu.value.style.display == "block") {
        menu.value.style.display = "none";
    } else {
        menu.value.style.display = "block";
    }
}

function closeMenu() {
    menu.value.style.display = "none";
}

function openDeleteDialog(id: string) {
    new WebviewWindow("deleteExtensionDialog", {
        resizable: false,
        transparent: true,
        center: true,
        height: 140,
        width: 500,
        title: "Delete Extension",
        url: `delete-extension-dialog?id=${id}`
    })
}

function openCommunityExtensionsDialog() {
    closeMenu();

    new WebviewWindow("communityExtensionsDialog", {
        width: 800,
        height: 800,
        transparent: true,
        center: true,
        url: "community-extensions-dialog",
        title: "Community Extensions"
    })
}

function importExtension() {
    closeMenu();

    new WebviewWindow("importExtensionDialog", {
        url: "import-extension-dialog",
        width: 800,
        height: 225,
        center: true,
        resizable: false,
        transparent: true,
        title: "Import Extension"
    })
}

async function restoreKeyword(extensionID: string) {
    let defaultValue: string = await invoke("get_extension_default_keyword", {
        extension_id: extensionID
    });

    (document.getElementById(`keyword-${extensionID}`) as HTMLInputElement).value = defaultValue;

    updateExtensionKeyword(extensionID, defaultValue);
}

async function restoreSetting(extensionID: string, settingID: string, type: "input" | "select") {
    let defaultValue: string = await invoke("get_extension_default_setting", {
        extension_id: extensionID,
        setting_id: settingID
    });

    (document.getElementById(`${type}-${extensionID}-${settingID}`) as HTMLInputElement).value = defaultValue;

    updateSetting(extensionID, settingID, defaultValue);
}

</script>

<template>
    <div class="p-4">
        <div class="flex">

            <div class="text-3xl ml-3 mb-4">Extensions</div>
            <div class="flex flex-grow justify-end">
                <button class="h-[50px] w-[50px] flex items-center justify-center menuButton" @click="toggleIconMenu()">
                    <ThreeDotsSVG class="h-5 w-5 menuButtonIcon" />
                </button>
                <div class="menu-content" ref="menu">
                    <button class="w-full p-2 flex justify-start hover:opacity-80 focus:opacity-80"
                        @click="importExtension()">Import
                        Extension</button>
                    <button class="w-full p-2 flex justify-start hover:opacity-80 focus:opacity-80"
                        @click="openCommunityExtensionsDialog()">
                        Community Extensions
                    </button>
                </div>
            </div>
        </div>

        <div v-for="extension in tabExtensions" class="p-4 secondaryBackground rounded-3xl mb-2">

            <div class="flex">
                <div class="font-bold text-lg ml-3">{{ extension.name }}</div>
                <div class="flex-grow"></div>
                <div class="p-2 pr-4 pl-4 rounded-full tertiaryBackground ml-4">
                    {{ extension.version }}
                </div>
                <div class="ml-4">
                    <button class="deleteButton" @click="openDeleteDialog(extension.id)">
                        <TrashSVG class="w-5 h-5 deleteIcon" />
                    </button>
                </div>
            </div>

            <div>
                <div class="ml-3 mt-2">Keyword</div>
                <div class="flex items-center">
                    <input class="input" :id="`keyword-${extension.id}`" placeholder="Extension Keyword" maxlength="10"
                        :value="getExtensionKeyword(extension.id)"
                        @change="event => updateExtensionKeyword(extension.id, (event.target as HTMLInputElement).value)">

                    <RestoreButton class="ml-2" @click="restoreKeyword(extension.id)" />
                </div>
            </div>

            <div v-if="os === 'linux'">
                <div class="mt-2" v-for="setting in extension.settings?.linux">
                    <div v-if="canShowSetting(extension.id, setting.id)">
                        <div class="ml-3">{{ setting.name }}</div>

                        <div v-if="setting.input === 'select'" class="flex">
                            <div class="flex w-full selectBox">
                                <select class="dropdown flex-grow" :id="`select-${extension.id}-${setting.id}`"
                                    :value="getSettingValue(extension.id, setting.id)"
                                    @change="event => updateSetting(extension.id, setting.id, (event.target as HTMLSelectElement).value)">
                                    <option v-for="option in setting.options" :value="option.value" :key="option.value">
                                        <div>{{ option.name }}</div>
                                    </option>
                                </select>
                                <div class="flex items-center justify-center chevron">
                                    <ChevronDownSVG class="h-3 w-3 fill" />
                                </div>
                            </div>

                            <RestoreButton class="ml-2" @click="restoreSetting(extension.id, setting.id, 'select')" />
                        </div>

                        <div v-if="setting.input === 'text'" class="flex">
                            <input class="input" :id="`input-${extension.id}-${setting.id}`"
                                :value="getSettingValue(extension.id, setting.id)"
                                @change="event => updateSetting(extension.id, setting.id, (event.target as HTMLInputElement).value)">

                            <RestoreButton class="ml-2" @click="restoreSetting(extension.id, setting.id, 'input')" />
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div></div>
    </div>
</template>

<style scoped>
.border {
    border: 1px solid v-bind(tertiaryBackgroundColor);
}

.accentText {
    color: v-bind(accentColor);
}

.input:focus {
    outline-color: 1px solid v-bind(accentColor);
}

.input {
    border-radius: 48px;
    padding: 8px;
    padding-left: 16px;
    padding-right: 16px;
    background-color: v-bind(tertiaryBackgroundColor);
    width: 100%;
}

.input::placeholder {
    color: v-bind(secondaryTextColor);
}

.fill {
    fill: v-bind(textColor);
}

.deleteButton {
    background-color: v-bind(dangerColor);
    padding: 8px;
    border-radius: 48px;
}

.deleteButton:hover {
    opacity: 0.8;
}

.deleteButton:focus {
    opacity: 0.8;
    border-radius: 14px;

}

.deleteIcon {
    stroke: v-bind(onDangerColor);
    stroke-width: 2px;
}


.selectBox {
    background-color: v-bind(tertiaryBackgroundColor);
    border: 1px solid v-bind(tertiaryBackgroundColor);
    border-radius: 48px;
}


.chevron {
    padding: 8px;
    padding-right: 16px;
    border-top-right-radius: 48px;
    border-bottom-right-radius: 48px;
}

.dropdown {
    all: unset;
    display: flex;
    width: 100%;
    padding: 8px;
    padding-left: 16px;
    padding-right: 16px;
    cursor: pointer;
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.tertiaryBackground {
    background-color: v-bind(tertiaryBackgroundColor);
}

.stroke {
    fill: none;
    stroke: v-bind(textColor);
    stroke-width: 2
}

.menuButton {
    background-color: v-bind(secondaryBackgroundColor);
    border-radius: 48px;
}

.menuButton:hover {
    outline: 2px solid v-bind(accentColor);
}

.menuButton:focus {
    outline: 2px solid v-bind(accentColor);
}

.menuButtonIcon {
    fill: v-bind(accentColor);
}

.menu-content {
    margin-top: 60px;
    margin-right: 20px;
    display: none;
    position: absolute;
    background-color: v-bind(tertiaryBackgroundColor);
    padding: 10px;
    z-index: 9999;
    right: 0;
    border-radius: 14px;
    outline: 2px solid v-bind(accentColor);
}
</style>