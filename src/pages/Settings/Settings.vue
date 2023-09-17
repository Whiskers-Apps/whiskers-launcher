<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { SettingsTab, getTheme } from "./Settings"
import GeneralTab from './GeneralTab.vue';
import ThemesTab from './ThemesTab.vue';
import SearchBoxTab from './SearchBoxTab.vue'
import SearchEnginesTab from './SearchEnginesTab.vue';
import ExtensionsTab from './ExtensionsTab.vue';
import { listen } from '@tauri-apps/api/event';
import AboutTab from './AboutTab.vue';
import Navbar from "./Navbar.vue";

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
                    <GeneralTab :background-color="backgroundColor" :secondary-background-color="secondaryBackgroundColor"
                        :accent-color="accentColor" :text-color="textColor"
                        :tertiary-background-color="tertiaryBackgroundColor" />
                </div>

                <div v-if="currentTab === SettingsTab.SearchBox">
                    <SearchBoxTab :background-color="backgroundColor" :secondary-background-color="secondaryBackgroundColor"
                        :tertiary-background-color="tertiaryBackgroundColor" :accent-color="accentColor" />
                </div>

                <div v-if="currentTab === SettingsTab.Theme">
                    <ThemesTab />
                </div>
                <div v-if="currentTab === SettingsTab.SearchEngines">
                    <SearchEnginesTab :background-color="backgroundColor"
                        :secondary-background-color="secondaryBackgroundColor"
                        :tertiary-background-color="tertiaryBackgroundColor" :accent-color="accentColor"
                        :text-color="textColor" :on-accent-color="onAccentColor" />
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
</style>

<style scoped>


.text {
    color: v-bind(textColor);
}

.background {
    background-color: v-bind(backgroundColor);
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
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



.fillAccent {
    fill: v-bind(accentColor);
    stroke: none;
}

.strokeAccent {
    fill: none;
    stroke: v-bind(accentColor);
    stroke-width: 2;
}
</style>