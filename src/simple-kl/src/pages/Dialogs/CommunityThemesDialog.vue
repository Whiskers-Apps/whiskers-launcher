<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '@pages/Settings/Settings';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { open as openLink } from "@tauri-apps/api/shell"
import BranchSVG from "@icons/branch.svg"
import CheckSVG from "@icons/check.svg"
import BrushSVG from "@icons/brush.svg"
import ChevronRightSVG from "@icons/chevron-right.svg"
import ChevronLeftSVG from "@icons/chevron-left.svg"

interface CommunityTheme {
    repo: string,
    file: string,
    preview: string,
    name: string
}

const backgroundColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const textColor = ref("");
const secondaryTextColor = ref("");
const accentColor = ref("");
const updateThemeEmit = ref();
const searchInput = ref("");
const disableButtons = ref(false);

const themesGrid = ref();
const currentPage = ref(1);
const themes = ref<CommunityTheme[]>([]);
const currentThemes = ref<CommunityTheme[]>([]);
const pageThemes = ref<CommunityTheme[]>([])

onMounted(async () => {
    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    });

    themes.value = await invoke("get_community_themes");
    currentThemes.value = themes.value;
    filterByPage();
})

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
    const endIndex = Math.min(startIndex + 20, currentThemes.value.length);

    pageThemes.value = currentThemes.value.slice(startIndex, endIndex);
    themesGrid.value.scrollTop = 0;
}

function filter() {
    currentPage.value = 1
    currentThemes.value = themes.value.filter((theme) => theme.name.toLowerCase().trim().includes(searchInput.value.toLowerCase().trim()));
    filterByPage()
}

async function applyTheme(repo: string, file: string) {
    disableButtons.value = true;
    await invoke("apply_community_theme", { repo: repo, file: file });
    disableButtons.value = false;
}

function getPagesCount(): number {
    return currentThemes.value.length / 20 + (currentThemes.value.length % 20 ? 1 : 0)
}
</script>
<template>
    <div class="main h-screen max-h-screen grid grid-cols-12">

        <div class="2xl:col-span-2 2xl:block xl:col-span-2 xl:block lg:col-span-1 lg:block hidden"></div>

        <div class="2xl:col-span-10 xl:col-span-10 lg:col-span-11 col-span-12 flex-grow flex flex-col overflow-auto">
            <div class="text-3xl ml-2">Community Themes</div>

            <input class="input mt-2" placeholder="Search for themes" v-model="searchInput" v-on:keyup.enter="filter()"
                @input="if (searchInput.length === 0) { filter(); }">

            <div class=" flex-grow max-w-[900px] overflow-auto mt-4" ref="themesGrid">
                <div v-for="(theme, index) in pageThemes" :key="index" class="grid grid-cols-2 themeCard mb-1 h-[300px]">
                    <div class="flex justify-center items-center ">
                        <img :src="theme.preview" class=" object-contain rounded-lg">
                    </div>
                    <div class="flex flex-col ml-4 text-center">
                        <div class="min-w-0 mb-4 oneLineText text-lg w-full">{{ theme.name }}</div>
                        <div class="flex-grow flex flex-col items-center justify-center">
                            <button class="cardButton w-full" :disabled="disableButtons" @click="openLink(theme.repo)">
                                <div class="flex justify-center items-center">
                                    <BranchSVG class="h-5 w-5 mr-2 fillIcon" />
                                    Source
                                </div>
                            </button>

                            <button class="cardButton w-full mt-4" :disabled="disableButtons"
                                @click="applyTheme(theme.repo, theme.file)">
                                <div class="flex justify-center items-center">
                                    <CheckSVG class="h-5 w-5 mr-2 applyIcon" />
                                    Apply
                                </div>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            <div class="mt-4 pageSelector flex">
                <button class="pageSelectorButton mr-4 " :disabled="currentPage === 1"
                    :class="currentPage === 1 ? 'disabled' : ''" @click="currentPage -= 1; filterByPage(); themesGrid.scrollTop = 0;">
                    <ChevronLeftSVG class="h-5 w-5 pageSelectorButtonIcon" />
                </button>
                <div class="tertiaryBackground p-2 flex text-center rounded-lg">
                    <div class="h-5 w-5">
                        {{ currentPage }}
                    </div>
                </div>
                <button class="pageSelectorButton ml-4" :class="currentPage + 1 > getPagesCount() ? 'disabled' : ''"
                    :disabled="currentPage + 1 > getPagesCount()" @click="currentPage += 1; filterByPage(); themesGrid.scrollTop = 0;">
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


.applyIcon {
    stroke: v-bind(textColor);
    stroke-width: 3px;
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