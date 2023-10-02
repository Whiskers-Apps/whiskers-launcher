<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '@pages/Settings/Settings';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { open as openLink } from "@tauri-apps/api/shell"
import BranchSVG from "@icons/branch.svg"
import DownloadSVG from "@icons/download.svg"
import TrashSVG from "@icons/trash.svg"
import { ExtensionManifest, getExtensions } from '@/data';
import { WebviewWindow } from '@tauri-apps/api/window';
import ChevronRightSVG from "@icons/chevron-right.svg"
import ChevronLeftSVG from "@icons/chevron-left.svg"

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
const disableButtons = ref(false);

const extensionsGrid = ref()
const currentPage = ref(1);
const extensions = ref<CommunityExtension[]>([]);
const currentExtensions = ref<CommunityExtension[]>([]);
const pageExtensions = ref<CommunityExtension[]>([]);

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
    filterByPage()
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

function filterByPage() {
    const startIndex = (currentPage.value - 1) * 20;
    const endIndex = Math.min(startIndex + 20, currentExtensions.value.length);

    pageExtensions.value = currentExtensions.value.slice(startIndex, endIndex);
    extensionsGrid.value.scrollTop = 0;
}

function filter() {
    currentPage.value = 1
    currentExtensions.value = extensions.value.filter((extension) => extension.name.toLowerCase().trim().includes(searchInput.value.toLowerCase().trim()) || extension.description.toLowerCase().trim().includes(searchInput.value.toLowerCase().trim()));
    filterByPage()
}



async function installExtension(extension: CommunityExtension) {
    disableButtons.value = true;
    await invoke("install_community_extension", { id: extension.id, repo: extension.repo });
    disableButtons.value = false;
}

function getPagesCount(): number {
    return currentExtensions.value.length / 20 + (currentExtensions.value.length % 20 ? 1 : 0)
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
    <div class="main h-screen max-h-screen grid grid-cols-12">

        <div class="2xl:col-span-2 2xl:block xl:col-span-2 xl:block lg:col-span-1 lg:block hidden "></div>

        <div class="2xl:col-span-10 xl:col-span-10 lg:col-span-12 col-span-12 flex-grow flex flex-col overflow-auto">
            <div class="text-3xl ml-2">Community Extensions</div>

            <input class="input mt-2" placeholder="Search for extensions" v-model="searchInput" v-on:keyup.enter="filter()"
                @input="if (searchInput.length === 0) { filter(); }">

            <div class=" flex-grow max-w-[900px] overflow-auto mt-4 grid grid-cols-2 gap-4" ref="extensionsGrid">
                <div v-for="(extension, index) in pageExtensions" :key="index" class="h-fit themeCard">
                    <div class="flex justify-center items-center">
                        <img :src="extension.preview" class="h-[200px] object-contain rounded-lg">
                    </div>
                    <div class="">
                        <div class="min-w-0 ml-1 mt-4 mb-2 oneLineText text-lg font-medium w-full">{{ extension.name }}
                        </div>
                        <div class="mb-4 ml-1">{{ extension.description }}</div>
                        <div class="flex-grow flex flex-col items-center justify-center">
                            <button class="cardButton w-full" :disabled="disableButtons" @click="openLink(extension.repo)">
                                <div class="flex justify-center items-center">
                                    <BranchSVG class="h-5 w-5 mr-2 fillIcon" />
                                    Source
                                </div>
                            </button>

                            <button v-if="isExtensionInstalled(extension)" class="cardButton w-full mt-4"
                                :disabled="disableButtons" @click="openDeleteDialog(extension)">
                                <div class="flex justify-center items-center">
                                    <TrashSVG class="h-5 w-5 mr-2 strokeIcon stroke-2" />
                                    Uninstall
                                </div>
                            </button>

                            <button v-else class="cardButton w-full mt-4" :disabled="disableButtons"
                                @click="installExtension(extension)">
                                <div class="flex justify-center items-center">

                                    <DownloadSVG class="h-5 w-5 mr-2 strokeIcon" />
                                    Install

                                </div>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            <div class="mt-4 pageSelector flex">
                <button class="pageSelectorButton mr-4 " :disabled="currentPage === 1"
                    :class="currentPage === 1 ? 'disabled' : ''"
                    @click="currentPage -= 1; filterByPage(); extensionsGrid.scrollTop = 0;">
                    <ChevronLeftSVG class="h-5 w-5 pageSelectorButtonIcon" />
                </button>
                <div class="tertiaryBackground p-2 flex text-center rounded-lg">
                    <div class="h-5 w-5">
                        {{ currentPage }}
                    </div>
                </div>
                <button class="pageSelectorButton ml-4" :class="currentPage + 1 > getPagesCount() ? 'disabled' : ''"
                    :disabled="currentPage + 1 > getPagesCount()"
                    @click="currentPage += 1; filterByPage(); extensionsGrid.scrollTop = 0;">
                    <ChevronRightSVG class="h-5 w-5 pageSelectorButtonIcon" />
                </button>
            </div>
        </div>

    </div>
</template>
<style scoped>
::-webkit-scrollbar {
    width: 6px;
    height: 6px;
    border-radius: 48px;
}

::-webkit-scrollbar-track {
    background: v-bind(tertiaryBackgroundColor);
    border-radius: 48px;
}

::-webkit-scrollbar-thumb {
    background: v-bind(accentColor);
    border-radius: 48px;
}

.tertiaryBackground {
    background-color: v-bind(tertiaryBackgroundColor);
}

.main {
    color: v-bind(textColor);
    background-color: v-bind(backgroundColor);
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
}

.input::placeholder {
    color: v-bind(secondaryTextColor);
}

input:focus {
    outline: 2px solid v-bind(accentColor);
}

.themeCard {
    padding: 16px;
    background-color: v-bind(secondaryBackgroundColor);
    border-radius: 24px;
}


.cardButton {
    background-color: v-bind(tertiaryBackgroundColor);
    padding: 8px;
    border-radius: 48px;
}

.cardButton:disabled {
    background-color: v-bind(tertiaryBackgroundColor);
}

.cardButton:hover:enabled {
    filter: brightness(0.95);
}

.fillIcon {
    fill: v-bind(textColor);
}

.strokeIcon {
    stroke: v-bind(textColor);
    stroke-width: 2px;
}

.pageSelector {
    padding: 8px;
    border-radius: 16px;
    background-color: v-bind(secondaryBackgroundColor);
    width: fit-content;
}

.pageSelectorButton {
    padding: 8px;
    background-color: v-bind(tertiaryBackgroundColor);
    border-radius: 8px;
}

.pageSelectorButton:hover:enabled,
.pageSelectorButton:focus:enabled {
    opacity: 0.9;
}

.pageSelectorButtonIcon {
    stroke: v-bind(textColor);
    stroke-width: 2;
}

.disabled {
    opacity: 0.2;
}
</style>