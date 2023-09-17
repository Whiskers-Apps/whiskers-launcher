<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '@pages/Settings/Settings';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { open as openLink } from "@tauri-apps/api/shell"
import BranchSVG from "@icons/branch.svg"
import CheckSVG from "@icons/check.svg"
import BrushSVG from "@icons/brush.svg"

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
const themes = ref<CommunityTheme[]>([]);
const currentThemes = ref<CommunityTheme[]>([]);
const disableButtons = ref(false);

onMounted(async () => {
    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    });

    themes.value = await invoke("get_community_themes");
    currentThemes.value = themes.value;
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

function filter() {
    console.log(`entrou ${searchInput.value}`)
    currentThemes.value = themes.value.filter((theme) => theme.name.toLowerCase().trim().includes(searchInput.value.toLowerCase().trim()));
}

async function applyTheme(repo: string, file: string) {
    disableButtons.value = true;
    await invoke("apply_community_theme", { repo: repo, file: file });
    disableButtons.value = false;
}
</script>
<template>
    <div class="main flex flex-col">
        <div class="flex items-center">
            <div class="text-3xl oneLineText">Community Themes</div>
            <BrushSVG class="h-7 w-7 ml-4 fillIcon" />
            <div class="flex-grow"></div>
            <input class="input" placeholder="Search for themes" v-model="searchInput" @input="filter()">
        </div>
        <div class="mt-4 flex-grow overflow-auto grid grid-flow-row 2xl:grid-cols-2 xl:grid-cols-2 md:grid-cols-2 grid-cols-1 gap-4">
            <div v-for="(theme, index) in currentThemes" :key="index" class="grid grid-cols-2 themeCard max-h-[200px]">
                <img :src="theme.preview" class=" object-contain h-[200px]">
                <div class="flex flex-col ml-4 ">
                    <div class="min-w-0 oneLineText text-lg w-full">{{ theme.name }}</div>
                    <div class="flex-grow flex flex-col items-center justify-center">
                        <button class="cardButton w-full" :disabled="disableButtons" @click="openLink(theme.repo)">
                            <div class="flex justify-center items-center">
                                <BranchSVG class="h-5 w-5 mr-2 fillIcon"/>
                                Source
                            </div>
                        </button>

                        <button class="cardButton w-full mt-4" :disabled="disableButtons" @click="applyTheme(theme.repo, theme.file)">
                            <div class="flex justify-center items-center">
                                <CheckSVG class="h-5 w-5 mr-2 applyIcon"/>
                                Apply
                            </div>
                        </button>
                    </div>
                </div>
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

.main {
    height: 100vh;
    width: 100vw;
    max-height: 100vh;
    max-width: 100vw;
    color: v-bind(textColor);
    background-color: v-bind(backgroundColor);
    padding: 16px;
}

.input {
    width: 100%;
    max-width: 400px;
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
</style>