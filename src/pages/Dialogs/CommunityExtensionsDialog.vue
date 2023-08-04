<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '@pages/Settings/Settings';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { open as openLink } from "@tauri-apps/api/shell"
import BranchSVG from "@icons/branch.svg"
import CheckSVG from "@icons/check.svg"
import DownloadSVG from "@icons/download.svg"
import TrashSVG from "@icons/trash.svg"
import { ExtensionManifest, getExtensions } from '@/data';
import { WebviewWindow } from '@tauri-apps/api/window';

interface CommunityExtension {
    id: string,
    name: string
    description: string,
    repo: string,
    preview: string,
}

const backgroundColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const textColor = ref("");
const secondaryTextColor = ref("");
const accentColor = ref("");
const updateThemeEmit = ref();
const updateExtensionsEmit = ref();
const searchInput = ref("");
const installedExtensions = ref<ExtensionManifest[]>([]);
const extensions = ref<CommunityExtension[]>([]);
const currentExtensions = ref<CommunityExtension[]>([]);
const disableButtons = ref(false);

onMounted(async () => {
    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    });

    loadExtensions();

    updateExtensionsEmit.value = listen("updateExtensions", (_event) => {
        loadExtensions();
    });
})

async function loadExtensions() {
    installedExtensions.value = await getExtensions();
    extensions.value = await invoke("get_community_extensions");
    currentExtensions.value = extensions.value;
}

async function loadTheme() {
    let theme = await getTheme();
    backgroundColor.value = theme.background;
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    accentColor.value = theme.accent;
    textColor.value = theme.text;
    secondaryTextColor.value = theme.secondary_text;
}

function filter() {
    currentExtensions.value = extensions.value.filter((extension) => extension.name.toLowerCase().trim().includes(searchInput.value.toLowerCase().trim()) || extension.description.toLowerCase().trim().includes(searchInput.value.toLowerCase().trim()));
}

async function installExtension(extension: CommunityExtension) {
    disableButtons.value = true;
    await invoke("install_community_extension", { id: extension.id, repo: extension.repo });
    disableButtons.value = false;
}

function isExtensionInstalled(extension: CommunityExtension) {

    if (installedExtensions.value.find((it) => it.id === extension.id) !== undefined) {
        return true
    } else {
        return false
    }
}

function openDeleteDialog(extension: CommunityExtension) {

    new WebviewWindow("deleteExtensionDialog", {
        resizable: false,
        transparent: true,
        center: true,
        height: 140,
        width: 500,
        title: "Delete Extension",
        url: `delete-extension-dialog?id=${extension.id}`
    })
}

</script>
<template>
    <div class="main flex flex-col">
        <div class="flex items-center">
            <div class="text-3xl">Community Extensions</div>
            <div class="flex-grow flex justify-end ml-4">
                <input class="input" v-model="searchInput" @input="filter()" placeholder="Search Extensions">
            </div>
        </div>
        <div class="grid 2xl:grid-cols-2 xl:grid-cols-2 lg:grid-cols-2 grid-cols-1 gap-4 mt-4 overflow-scroll ">
            <div class="extensionCard grid grid-cols-3 col-span-1" v-for="(extension, index) in currentExtensions"
                :key="index">
                <div class="col-span-1">
                    <img :src="extension.preview" class="h-[200px] object-contain">
                </div>
                <div class="h-full flex col-span-2 ml-4">
                    <div class="flex-grow overflow-hidden whitespace-break-spaces text-start">
                        <div class="col-span-2 oneLineText text-lg font-bold">
                            {{ extension.name }}
                        </div>
                        <div class="mt-4 h-full">
                            {{ extension.description }}
                        </div>
                    </div>

                    <div class="flex items-center">
                        <button class="cardButton ml-4" :disabled="disableButtons" @click="openLink(extension.repo)">
                            <BranchSVG class="h-6 w-6 fillIcon" />
                        </button>
                        <button v-if="isExtensionInstalled(extension)" @click="openDeleteDialog(extension)"
                            class="cardButton ml-4" :disabled="disableButtons">
                            <TrashSVG class="h-6 w-6 strokeIcon" />
                        </button>
                        <button v-else class="ml-4 cardButton" @click="installExtension(extension)"
                            :disabled="disableButtons">
                            <DownloadSVG class="h-6 w-6 strokeIcon" />
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
<style scoped>
.main {
    height: 100vh;
    width: 100vw;
    max-height: 100vh;
    max-width: 100vw;
    background-color: v-bind(backgroundColor);
    color: v-bind(textColor);
    padding: 16px;
}

.input {
    width: 100%;
    max-width: 500px;
    padding: 8px;
    padding-left: 16px;
    padding-right: 16px;
    background-color: v-bind(tertiaryBackgroundColor);
    border-radius: 48px;
    outline: 1px solid v-bind(textColor);
}

.input::placeholder {
    color: v-bind(secondaryTextColor);
}

input:focus {
    outline: 2px solid v-bind(accentColor);
}

.extensionCard {
    padding: 16px;
    background-color: v-bind(secondaryBackgroundColor);
    border-radius: 24px;
}


.cardButton {
    background-color: v-bind(tertiaryBackgroundColor);
    padding: 16px;
    border-radius: 48px;
}

.cardButton:disabled {
    background-color: v-bind(tertiaryBackgroundColor);
}

.cardButton:hover:enabled {
    outline: 2px solid v-bind(accentColor);
}

.fillIcon {
    fill: v-bind(textColor);
}

.strokeIcon {
    stroke: v-bind(textColor);
    stroke-width: 2px;
}
</style>