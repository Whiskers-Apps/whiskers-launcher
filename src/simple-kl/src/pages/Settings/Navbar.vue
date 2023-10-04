<script setup lang="ts">

import { onMounted, PropType, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import SearchSVG from "@icons/search.svg"
import HomeSVG from "@icons/home.svg"
import BrushSVG from "@icons/brush.svg"
import PluginSVG from "@icons/plugin.svg"
import InfoSVG from "@icons/info.svg"
import MenuSVG from "@icons/menu.svg"

import { getTheme, SettingsTab } from '@pages/Settings/Settings';

const accentColor = ref("");
const textColor = ref("");
const secondaryTextColor = ref("");
const tertiaryBackgroundColor = ref("")
const updateThemeEmit = ref();
const showNavBar = ref(true);

const emit = defineEmits(["click"])

defineProps({
    currentTab: {
        required: true,
        type: Object as PropType<SettingsTab>
    }
});


onMounted(async () => {
    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    })
});

async function loadTheme() {
    let theme = await getTheme();

    accentColor.value = theme.accent;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    textColor.value = theme.text;
    secondaryTextColor.value = theme.secondary_text;
}
</script>

<template>
    <div class=" h-screen flex flex-col  p-4">

        <button class="p-2 ml-2 navbarToggle w-fit" @click="showNavBar = !showNavBar">
            <MenuSVG class="h-7 w-7 navbarToggleIcon" />
        </button>

        <div v-if="showNavBar" class="min-w-[250px] overflow-auto">
            <div class="mt-4 tab flex items-center" v-bind:class="currentTab === SettingsTab.General ? 'activeTab' : ''"
                @click="emit('click', SettingsTab.General)">
                <div class="">
                    <HomeSVG class="h-6 w-6 strokeAccent" />
                </div>

                <div class="ml-3">
                    General
                </div>

            </div>
            <div class="tab flex items-center" v-bind:class="currentTab === SettingsTab.Search ? 'activeTab' : ''"
                @click="emit('click', SettingsTab.Search)">
                <div class="">
                    <SearchSVG class="h-6 w-6 strokeAccent" />
                </div>
                <div class="ml-3">
                    Search
                </div>
            </div>

            <div class="tab flex items-center" v-bind:class="currentTab === SettingsTab.Results ? 'activeTab' : ''"
                @click="emit('click', SettingsTab.Results)">
                <div class="">
                    <SearchSVG class="h-6 w-6 strokeAccent" />
                </div>
                <div class="ml-3">
                    Results
                </div>
            </div>

            <div class="tab flex items-center" v-bind:class="currentTab === SettingsTab.SearchEngines ? 'activeTab' : ''"
                @click="emit('click', SettingsTab.SearchEngines)">
                <div class="">
                    <SearchSVG class="h-6 w-6 strokeAccent" />
                </div>
                <div class="ml-3">
                    Search Engines
                </div>
            </div>

            <div class="tab flex items-center" v-bind:class="currentTab === SettingsTab.Theme ? 'activeTab' : ''"
                @click="emit('click', SettingsTab.Theme)">
                <div class="">
                    <BrushSVG class="h-6 w-6 fillAccent" />
                </div>
                <div class="ml-3">
                    Theming
                </div>
            </div>

            <div class="tab flex items-center" v-bind:class="currentTab === SettingsTab.Extensions ? 'activeTab' : ''"
                @click="emit('click', SettingsTab.Extensions)">
                <div class="">
                    <PluginSVG class="h-6 w-6 fillAccent" />
                </div>
                <div class="ml-3">
                    Extensions
                </div>
            </div>

            <div class="tab flex items-center" v-bind:class="currentTab === SettingsTab.About ? 'activeTab' : ''"
                @click="emit('click', SettingsTab.About)">
                <div class="">
                    <InfoSVG class="h-6 w-6 fillAccent" />
                </div>
                <div class="ml-3">
                    About
                </div>
            </div>
        </div>


    </div>
</template>

<style scoped>
.navbarToggle:hover {
    background-color: v-bind(tertiaryBackgroundColor);
    border-radius: 48px;
}

.navbarToggleIcon {
    stroke: v-bind(textColor);
}

.fillAccent {
    fill: v-bind(textColor);
    stroke: none;
}

.strokeAccent {
    fill: none;
    stroke: v-bind(textColor);
    stroke-width: 2;
}

.tab {
    padding: 16px;
    cursor: pointer;
    border-radius: 48px;
    color: v-bind(secondaryTextColor);
}

.tab:hover {
    color: v-bind(textColor);
}

.activeTab {
    background-color: v-bind(tertiaryBackgroundColor);
    font-weight: 500;
}
</style>