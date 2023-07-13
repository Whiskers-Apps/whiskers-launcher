<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { SearchOption, Settings, getSettings, ExtensionSettings as SettingsExtensionsSettings } from './Settings';
import { invoke } from '@tauri-apps/api';
import { ExtensionManifest, ExtensionSettings } from '../../data';

const settings = ref<Settings>();
const installedExtensions = ref<SearchOption[]>();
const tabExtensions = ref<TabExtensionManifest[]>();
const os = ref("");
const extensionsRef = ref([]);
const extensionSettingsRef = ref([])
const extensionAnySettingsRef = ref([])
const extensionLinuxSettingsRef = ref([])
const extensionWindowsSettingsRef = ref([])

export interface TabExtensionManifest {
    id: string,
    name: string,
    description?: string,
    icon: string,
    os: string,
    keyword: string,
    current_keyword: string,
    settings: ExtensionSettings[],
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
    invoke("update_extension_setting", { extension_id: extensionID, setting_id: settingID, new_value: newValue })
    settings.value = await getSettings();
}

function canShowSetting(extensionID: string, settingID: string): boolean {

    var showSetting = true;


    return showSetting
}

async function getSettingValue(extensionID: string, settingID: string): Promise<string> {

    settings.value?.extensions.forEach((extension) => {
        if (extension.id == extensionID) {
            extension.settings.any.forEach((setting) => {
                if (setting.id == settingID) {
                    return setting.current_value;
                }
            })
            extension.settings.linux.forEach((setting) => {
                if (setting.id == settingID) {
                    return setting.current_value;
                }
            })
            extension.settings.windows.forEach((setting) => {
                if (setting.id == settingID) {
                    return setting.current_value;
                }
            })
        }
    })

    return ""
}

function getExtensionKeyword(extensionID: string): string {

    settings.value?.extensions.forEach((extension) => {
        if (extension.id == extensionID) {
            return extension.keyword
        }
    })

    return ""
}

function loadSettingValue(extensionID:string, settingID: string){

}

onMounted(async () => {
    settings.value = await getSettings();
    os.value = await invoke("get_os");
    let extensions: ExtensionManifest[] = JSON.parse(await invoke("get_extensions_json"));
    let newTabExtensions: TabExtensionManifest[] = [];

    extensions.forEach(extension=>{

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

    //console.log(extensionsRef.value);
    //console.log(extensionAnySettingsRef.value);
    //console.log(extensionLinuxSettingsRef.value);
    //console.log(extensionWindowsSettingsRef.value);
})



</script>

<template>
    <div>
        <div class="text-2xl font-bold">Installed Extensions</div>
        <div  v-for="extension in tabExtensions" class="p-2 secondaryBackground  rounded-2xl" ref="extensionsRef">
            <div class="font-bold text-lg">{{ extension.name }}</div>
            <div v-for="setting in extension.settings.any" ref="extensionAnySettingsRef">
                <div v-if="canShowSetting(extension.settings, setting.id)"></div>
            </div>
            <div v-if="os === 'linux'" v-for="setting in extension.settings.linux" ref="extensionLinuxSettingsRef">
                <div v-if="canShowSetting(extension.settings, setting.id)" class="p-2 tertiaryBackground mb-2">
                    <div class="flex-grow">
                        <div class="flex font-bold">{{ setting.name }}</div>
                        <div class="text-sm">{{ setting.description }}</div>
                        <div class="flex">
                            <select v-if="setting.input == 'select'" class="dropwdown"
                                :ref="`${extension.id} | ${setting.id}`"
                                @change="updateSetting(extension.id, setting.id, ($event.target as HTMLSelectElement).value)"
                                
                                >
                                
                                <option v-for="option in setting.options" :value="option.value">
                                    {{ option.name }}
                                </option>
                            </select>
                            <input v-if="setting.input == 'text'"
                                class="rounded-lg p-2 secondaryBackground outline-none input flex-grow">
                        </div>
                    </div>
                </div>

            </div>
            <div v-if="os === 'windows'" v-for="setting in extension.settings.windows" ref="extensionAnySettingsRef">
                <div v-if="canShowSetting(extension.settings, setting.id)"></div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.accentText {
    color: v-bind(accentColor);
}

.input:focus {
    border: 1px solid v-bind(accentColor);
}

.dropwdown {
    all: unset;
    display: flex;
    width: 100%;
    background-color: v-bind(secondaryBackgroundColor);
    border-radius: 8px;
    padding: 8px;
    border: 1px solid v-bind(accentColor);
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