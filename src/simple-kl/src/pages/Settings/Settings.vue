<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { SettingsTab, getTheme } from "@pages/Settings/Settings"
import { listen } from '@tauri-apps/api/event';

import GeneralTab from '@pages/Settings/GeneralTab.vue';
import ThemesTab from '@pages/Settings/ThemesTab.vue';
import SearchBoxTab from '@pages/Settings/SearchBoxTab.vue'
import ResultsTab from "@pages/Settings/ResultsTab.vue"
import SearchEnginesTab from '@pages/Settings/SearchEnginesTab.vue';
import ExtensionsTab from '@pages/Settings/ExtensionsTab.vue';
import AboutTab from '@pages/Settings/AboutTab.vue';
import Navbar from "@pages/Settings/Navbar.vue";

const currentTab = ref(SettingsTab.General);

const backgroundColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");
const onAccentColor = ref("");
const dangerColor = ref("");
const onDangerColor = ref("");
const textColor = ref("");
const secondaryTextColor = ref("");

const updateThemeEmit = ref();

onMounted(async () => {
    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    })
})

async function loadTheme() {
    let theme = await getTheme();

    backgroundColor.value = theme.background;
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    accentColor.value = theme.accent;
    onAccentColor.value = theme.on_accent;
    dangerColor.value = theme.danger;
    onDangerColor.value = theme.on_danger;
    textColor.value = theme.text;
    secondaryTextColor.value = theme.secondary_text;
}


</script>

<template>
    <div class="background text h-screen max-h-screen">
        <div class="flex">
            <Navbar :current-tab="currentTab" @click="currentTab = $event" />

            <div class="flex-grow ml-[40px] overflow-y-auto h-screen test">
                <div v-if="currentTab === SettingsTab.General">
                    <GeneralTab/>
                </div>

                <div v-if="currentTab === SettingsTab.Search">
                    <SearchBoxTab/>
                </div>

                <div v-if="currentTab === SettingsTab.Results">
                    <ResultsTab/>
                </div>

                <div v-if="currentTab === SettingsTab.Theme">
                    <ThemesTab />
                </div>
                <div v-if="currentTab === SettingsTab.SearchEngines">
                    <SearchEnginesTab/>
                </div>
                <div v-if="currentTab === SettingsTab.Extensions">
                    <ExtensionsTab :background-color="backgroundColor"
                        :secondary-background-color="secondaryBackgroundColor"
                        :tertiary-background-color="tertiaryBackgroundColor" :accent-color="accentColor"
                        :text-color="textColor" />
                </div>

                <div v-if="currentTab === SettingsTab.About">
                    <AboutTab />
                </div>
            </div>
        </div>

    </div>
</template>\

<style>

::-webkit-scrollbar {
    width: 6px;
}

::-webkit-scrollbar-track {
    background: v-bind(tertiaryBackgroundColor);
    border-radius: 48px;
}

::-webkit-scrollbar-thumb {
    background: v-bind(accentColor);
    border-radius: 48px;
}

.background {
    background-color: v-bind(backgroundColor);
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.tertiaryBackground {
    background-color: v-bind(tertiaryBackgroundColor);
}

.text {
    color: v-bind(textColor);
}

.fillAccent {
    fill: v-bind(accentColor);
    stroke: none;
}

.fillText{
    fill: v-bind(textColor);
}

.strokeText{
    stroke: v-bind(textColor);
}

.strokeAccent {
    fill: none;
    stroke: v-bind(accentColor);
    stroke-width: 2;
}
</style>

<style scoped>

html{
    background: black;
}

body{
    background-color: red;
}

.placeholder::placeholder {
    color: v-bind(secondaryTextColor);
}

::selection {
    background-color: v-bind(secondaryBackgroundColor);
}

.secondaryHover:hover {
    background-color: v-bind(secondaryBackgroundColor);
}

.tab {
    padding: 10px;
    border-radius: 10px;
    font-size: 17px;
}

.tab:hover {
    background-color: v-bind(tertiaryBackgroundColor);
    font-weight: 600;
    cursor: pointer;
}


</style>