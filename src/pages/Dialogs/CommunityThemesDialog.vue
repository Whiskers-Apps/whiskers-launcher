<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '@pages/Settings/Settings';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { open as openLink } from "@tauri-apps/api/shell"
import BranchSVG from "@icons/branch.svg"
import CheckSVG from "@icons/check.svg"

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
    console.log("entrou");
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
            <div class="text-3xl">Community Themes</div>
            <div class="flex-grow flex justify-end ml-4">
                <input class="input" v-model="searchInput" @input="filter()" placeholder="Search Themes">
            </div>
        </div>
        <div class="grid 2xl:grid-cols-2 xl:grid-cols-2 lg:grid-cols-2 grid-cols-1 gap-4 mt-4 overflow-scroll">
            <div class="themeCard grid grid-cols-3 col-span-1" 
                v-for="(theme, index) in currentThemes" :key="index">
                <div class="col-span-1">
                    <img :src="theme.preview" class="h-[200px] object-contain">
                </div>
                <div class="h-full flex col-span-2 items-center">
                    <div class="col-span-2 oneLineText ml-4 text-lg flex-grow text-start">{{ theme.name }}</div>
                    <button class="cardButton ml-4" :disabled="disableButtons" @click="openLink(theme.repo)">
                        <BranchSVG class="h-6 w-6 fillIcon" />
                    </button>
                    <button class="cardButton ml-4" :disabled="disableButtons" @click="applyTheme(theme.repo, theme.file)">
                        <CheckSVG class="h-6 w-6 applyIcon" />
                    </button>
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

.themeCard {
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

.cardButton:hover:enabled{
    outline: 2px solid v-bind(accentColor);
}

.fillIcon {
    fill: v-bind(textColor);
}

.applyIcon{
    stroke: v-bind(textColor);
    stroke-width: 3px;
}
</style>