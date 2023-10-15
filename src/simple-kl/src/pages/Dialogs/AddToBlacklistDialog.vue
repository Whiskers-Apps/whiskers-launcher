<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '../Settings/Settings';
import { invoke } from '@tauri-apps/api';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { emit } from "@tauri-apps/api/event";

import CheckSVG from "@icons/check.svg";
import ClearSVG from "@icons/clear.svg";

import PrimaryButton from '@/components/PrimaryButton.vue';
import { appWindow } from '@tauri-apps/api/window';

interface WhiteListApp {
    icon: string,
    name: string,
    path: string,
    checked: Boolean;
}


const backgroundColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const textColor = ref("");
const accentColor = ref("");
const onAccentColor = ref("");

const whiteListApps = ref<WhiteListApp[]>([]);
const checkedApps = ref<WhiteListApp[]>([])


onMounted(async () => {

    let theme = await getTheme();
    backgroundColor.value = theme.background;
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    textColor.value = theme.text;
    accentColor.value = theme.accent;
    onAccentColor.value = theme.on_accent;

    whiteListApps.value = await getWhiteListApps();
});

async function getWhiteListApps(): Promise<WhiteListApp[]> {

    let apps: WhiteListApp[] = await invoke("get_whitelist_apps");
    apps.sort((a, b) => a.name.localeCompare(b.name));

    return apps;
}


async function toggleApp(index: number) {

    whiteListApps.value[index].checked = !whiteListApps.value[index].checked;
    checkedApps.value = whiteListApps.value.filter(app => app.checked);
}

async function clearApps() {
    whiteListApps.value = await getWhiteListApps();
    checkedApps.value = [];
}

async function addToBlacklist() {

    checkedApps.value.forEach(async (app) => {
        await invoke("add_to_blacklist", { path: app.path });
    });

    emit("update-blacklist");

    appWindow.close();
}


</script>

<template>
    <div class="h-screen w-full max-h-screen overflow-auto main p-6 flex flex-col">
        <div class=" font-medium text-2xl">Add To Blacklist</div>
        <div>Click on the apps you would like to blacklist.</div>
        <button class="clearButton mt-4" @click="clearApps()">
            <div class="flex items-center">
                <ClearSVG class="mr-2 h-5 w-5 clearIcon" />
                Clear
            </div>
        </button>
        <div class="mt-4 flex-grow overflow-auto appList">
            <div v-for="(app, index) in whiteListApps" :key="index" class="flex flex-col" @click="toggleApp(index)">
                <div class="flex items-center appCard">
                    <img :src="convertFileSrc(app.icon)" class="h-8 w-8" />
                    <div class="ml-4 flex-grow mr-4">{{ app.name }}</div>
                    <CheckSVG v-if="app.checked" class="h-5 w-5 checkIcon" />
                </div>
                <div v-if="index < whiteListApps.length" class="divider"></div>
            </div>
        </div>
        <PrimaryButton class="mt-4" text="Add" :disabled="checkedApps.length === 0" :expand="true"
            @click="addToBlacklist()" />
    </div>
</template>

<style scoped>
::-webkit-scrollbar {
    width: 6px;
}

::-webkit-scrollbar-track {
    margin-top: 20px;
    margin-bottom: 20px;
    background: transparent;
    border-radius: 48px;
}

::-webkit-scrollbar-thumb {
    background: v-bind(accentColor);
    border-radius: 48px;
}

.main {
    background-color: v-bind(backgroundColor);
    color: v-bind(textColor);
}

.appList {
    background-color: v-bind(secondaryBackgroundColor);
    border-radius: 14px;
}

.appCard {
    padding: 24px;
}

.appCard:hover {
    cursor: pointer;
    background-color: v-bind(tertiaryBackgroundColor);
}

.checkIcon,
.clearIcon {
    stroke: v-bind(textColor);
    stroke-width: 2;
}


.divider {
    height: 2px;
    background-color: v-bind(tertiaryBackgroundColor);
    width: 100%;
}

.clearButton {
    background-color: v-bind(secondaryBackgroundColor);
    width: fit-content;
    padding: 12px;
    padding-left: 16px;
    padding-right: 16px;
    border-radius: 48px;
}

.clearButton:hover {
    background-color: v-bind(tertiaryBackgroundColor);
    outline: 1px solid v-bind(secondaryBackgroundColor);
}
</style>