<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '../Settings/Settings';
import { invoke } from '@tauri-apps/api';

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

const searchInput = ref("");
const themes = ref<CommunityTheme[]>([]);

onMounted(async () => {
    loadTheme()

    themes.value = await invoke("get_community_themes");
})

async function loadTheme() {
    let theme = await getTheme();
    backgroundColor.value = theme.background;
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    accentColor.value = theme.accent;
    textColor.value = theme.text;
    secondaryTextColor.value = theme.seconday_text;
}

function filter() {

}
</script>
<template>
    <div class="main overflow-scroll">
        <div class="flex items-center">
            <div class="text-3xl">Community Themes</div>
            <div class="flex-grow flex justify-end">
                <input class="input" v-model="searchInput" @input="filter()" placeholder="Search Themes">
            </div>       
        </div>
        <div class="grid grid-cols-6 gap-4 mt-4">
            <div class="themeCard aspect-square" v-for="theme in themes">
                <div class="flex items-center justify-center">
                    <img :src="theme.preview" class=" max-h-[140px] object-contain">
                    
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
}

.input::placeholder {
    color: v-bind(secondaryTextColor);
}

input:focus {
    outline: 2px solid v-bind(accentColor);
}

.themeCard{
    padding: 16px;
    background-color: v-bind(secondaryBackgroundColor);
    border-radius: 24px;
}
</style>