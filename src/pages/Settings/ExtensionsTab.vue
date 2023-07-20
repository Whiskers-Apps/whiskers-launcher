<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Settings, getSettings, ExtensionSettings as SettingsExtensionsSettings } from './Settings';
import { event, invoke } from '@tauri-apps/api';
import { ExtensionManifest, ExtensionSettings } from '../../data';
import ChevronDownSVG from "../../assets/icons/chevron-down.svg"

const settings = ref<Settings>();
const tabExtensions = ref<TabExtensionManifest[]>();
const os = ref("");

export interface TabExtensionManifest {
    id: string,
    name: string,
    description?: string,
    icon: string,
    os: string,
    keyword: string,
    current_keyword: string,
    settings: ExtensionSettings,
    current_settings: SettingsExtensionsSettings[]
}


defineProps({
    backgroundColor: {
        required: true,
        type: String
    },
    secondaryBackgroundColor: {
        required: true,
        type: String
    },
    tertiaryBackgroundColor: {
        required: true,
        type: String
    },
    accentColor: {
        required: true,
        type: String
    },
    textColor: {
        required: true,
        type: String
    }
})

async function updateSetting(extensionID: string, settingID: string, newValue: string) {
    console.log("banana")
    invoke("update_extension_setting", { extension_id: extensionID, setting_id: settingID, new_value: newValue })
    settings.value = await getSettings();
}

function canShowSetting(extensionID: string, settingID: string): boolean {

    let showSetting = false

    tabExtensions.value?.forEach(extension => {
        extension.settings.any.forEach(setting => {
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
        extension.settings.linux.forEach(setting => {
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
        extension.settings.windows.forEach(setting => {
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
            extension.settings.any.forEach((setting) => {
                if (setting.id == settingID) {
                    settingValue = setting.current_value;
                }
            })
            extension.settings.linux.forEach((setting) => {
                if (setting.id == settingID) {
                    settingValue = setting.current_value;
                }
            })
            extension.settings.windows.forEach((setting) => {
                if (setting.id == settingID) {
                    settingValue = setting.current_value;
                }
            })
        }
    })

    return settingValue
}

function getExtensionKeyword(extensionID: string): string {
    if (settings.value?.extensions?.find(extension => extension.id === extensionID)?.keyword !== null) {
        return settings.value!!.extensions.find(extension => extension.id === extensionID)!!.keyword
    } else {
        return ""
    };

}

async function updateExtensionKeyword(extensionID: string, keyword: string) {

    invoke("update_extension_keyword", { extension_id: extensionID, keyword: keyword });
    settings.value = await getSettings();
}

onMounted(async () => {
    settings.value = await getSettings();
    os.value = await invoke("get_os");
    getExtensions();
})

async function getExtensions() {

    settings.value = await getSettings();
    let extensions: ExtensionManifest[] = JSON.parse(await invoke("get_extensions_json"));
    let newTabExtensions: TabExtensionManifest[] = [];

    extensions.forEach(extension => {

        let newTabExtension: TabExtensionManifest = {
            id: extension.id,
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
    })

    tabExtensions.value = newTabExtensions;
}


</script>

<template>
    <div>

        <div class="text-xl">Installed Extensions</div>
        
        <div v-for="extension in tabExtensions" class="p-4 secondaryBackground  rounded-2xl mb-2">
            <div class="font-bold text-lg">{{ extension.name }}</div>

            <div>
                <div>Keyword</div>
                <div class="flex">
                    <input class="flex-grow tertiaryBackground rounded-lg p-2 input outline-none" maxlength="10"
                        :value="getExtensionKeyword(extension.id)"
                        @change="event => updateExtensionKeyword(extension.id, (event.target as HTMLInputElement).value)">
                </div>
            </div>

            <div v-if="os === 'linux'">
                <div v-for="setting in extension.settings.linux">
                    <div v-if="canShowSetting(extension.id, setting.id)">
                        <div>{{ setting.name }}</div>

                        <div v-if="setting.input === 'select'" class="flex">
                            <select class="dropdown flex-grow" :value="getSettingValue(extension.id, setting.id)"
                                @change="event => updateSetting(extension.id, setting.id, (event.target as HTMLSelectElement).value)">
                                <option v-for="option in setting.options" :value="option.value" :key="option.value">
                                    <div>{{ option.name }}</div>
                                </option>
                            </select>
                            <div class="flex items-center justify-center tertiaryBackground chevron p-2">
                                <ChevronDownSVG class="h-3 w-3 fill"/>
                            </div>
                        </div>

                        <div v-if="setting.input === 'text'" class="flex">
                            <input class="flex-grow tertiaryBackground rounded-lg p-2 outline-none input"
                                :value="getSettingValue(extension.id, setting.id)"
                                @change="event => updateSetting(extension.id, setting.id, (event.target as HTMLInputElement).value)">
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="text-xl">Online Extensions</div>

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
    border: 1px solid v-bind(accentColor);
}

.input {
    border: 1px solid v-bind(tertiaryBackgroundColor);
}

.fill{
    fill: v-bind(textColor);
}

.chevron{

    border-top-right-radius: 8px;
    border-bottom-right-radius: 8px;
}

.dropdown {
    all: unset;
    display: flex;
    width: 100%;
    background-color: v-bind(tertiaryBackgroundColor);
    border-top-left-radius: 8px;
    border-bottom-left-radius: 8px;
    padding: 8px;
    border: 1px solid v-bind(tertiaryBackgroundColor);
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





</style>