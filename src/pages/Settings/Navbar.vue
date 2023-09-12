<script setup lang="ts">

import { onMounted, PropType, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import SearchSVG from "@icons/search.svg"
import HomeSVG from "@icons/home.svg"
import BrushSVG from "@icons/brush.svg"
import PluginSVG from "@icons/plugin.svg"
import InfoSVG from "@icons/info.svg"

import { getTheme, SettingsTab } from '@pages/Settings/Settings';

const accentColor = ref("");
const tertiaryBackgroundColor = ref("")
const updateThemeEmit = ref();

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

}
</script>

<template>
    <div class=" max-w-fit h-screen secondaryBackground p-2 overflow-auto">
        <div class="tab flex flex-col items-center" v-bind:class="currentTab === SettingsTab.General ? 'activeTab' : ''"
            @click="emit('click', SettingsTab.General)">
            <div class="flex justify-center w-full">
                <HomeSVG class="h-7 w-7 strokeAccent" />
            </div>

            <div class="flex justify-center w-full text-sm">
                General
            </div>

        </div>
        <div class="tab mt-4 flex flex-col items-center"
            v-bind:class="currentTab === SettingsTab.SearchBox ? 'activeTab' : ''"
            @click="emit('click', SettingsTab.SearchBox)">
            <div class="w-full flex justify-center">
                <SearchSVG class="h-7 w-7 strokeAccent" />
            </div>
            <div class="w-full flex justify-center text-sm">
                Search Box
            </div>
        </div>

        <div class="tab mt-4 flex flex-col items-center"
            v-bind:class="currentTab === SettingsTab.SearchEngines ? 'activeTab' : ''"
            @click="emit('click', SettingsTab.SearchEngines)">
            <div class="w-full flex justify-center">
                <SearchSVG class="h-7 w-7 strokeAccent" />
            </div>
            <div class="w-full flex justify-center text-sm">
                Search Engines
            </div>
        </div>

        <div class="tab mt-4 flex flex-col items-center" v-bind:class="currentTab === SettingsTab.Theme ? 'activeTab' : ''"
            @click="emit('click', SettingsTab.Theme)">
            <div class="w-full flex justify-center">
                <BrushSVG class="h-7 w-7 fillAccent" />
            </div>
            <div class="w-full flex justify-center text-sm">
                Theming
            </div>
        </div>

        <div class="tab mt-4 flex flex-col items-center"
            v-bind:class="currentTab === SettingsTab.Extensions ? 'activeTab' : ''"
            @click="emit('click', SettingsTab.Extensions)">
            <div class="w-full flex justify-center">
                <PluginSVG class="h-7 w-7 fillAccent" />
            </div>
            <div class="w-full flex justify-center text-sm">
                Extensions
            </div>
        </div>

        <div class="tab mt-4 flex flex-col items-center" v-bind:class="currentTab === SettingsTab.About ? 'activeTab' : ''"
            @click="emit('click', SettingsTab.About)">
            <div class="w-full flex justify-center">
                <InfoSVG class="h-7 w-7 fillAccent" />
            </div>
            <div class="w-full flex justify-center text-sm">
                About
            </div>
        </div>
    </div>
</template>

<style scoped>
.fillAccent {
    fill: v-bind(accentColor);
    stroke: none;
}

.strokeAccent {
    fill: none;
    stroke: v-bind(accentColor);
    stroke-width: 2;
}

.tab {
    padding: 8px;
    cursor: pointer;
    border-radius: 14px;
}

.tab:hover{
    background-color: v-bind(tertiaryBackgroundColor);
    
}

.activeTab {
    background-color: v-bind(tertiaryBackgroundColor);
    font-weight: 600;
}
</style>