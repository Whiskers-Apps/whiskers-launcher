<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from './Settings';
import { SettingsTabs } from './Settings';
import SearchSVG from "../../assets/icons/search.svg"
import HomeSVG from "../../assets/icons/home.svg"
import BrushSVG from "../../assets/icons/brush.svg"
import PluginSVG from "../../assets/icons/plugin.svg"
import InfoSVG from "../../assets/icons/info.svg"
import { listen } from '@tauri-apps/api/event';


const accentColor = ref("");
const tertiaryBackgroundColor = ref("")
const updateThemeEmit = ref();

const emit = defineEmits(["click"])

defineProps({
    currentTab: {
        required: true,
        type: String
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
        <div class="tab flex flex-col items-center" v-bind:class="currentTab === SettingsTabs.GENERAL ? 'activeTab' : ''"
            @click="emit('click', SettingsTabs.GENERAL)">
            <div class="flex justify-center w-full">
                <HomeSVG class="h-7 w-7 strokeAccent" />
            </div>

            <div class="flex justify-center w-full text-sm">
                General
            </div>

        </div>
        <div class="tab mt-4 flex flex-col items-center"
            v-bind:class="currentTab === SettingsTabs.SEARCH_BOX ? 'activeTab' : ''"
            @click="emit('click', SettingsTabs.SEARCH_BOX)">
            <div class="w-full flex justify-center">
                <SearchSVG class="h-7 w-7 strokeAccent" />
            </div>
            <div class="w-full flex justify-center text-sm">
                Search Box
            </div>
        </div>

        <div class="tab mt-4 flex flex-col items-center"
            v-bind:class="currentTab === SettingsTabs.SEARCH_ENGINES ? 'activeTab' : ''"
            @click="emit('click', SettingsTabs.SEARCH_ENGINES)">
            <div class="w-full flex justify-center">
                <SearchSVG class="h-7 w-7 strokeAccent" />
            </div>
            <div class="w-full flex justify-center text-sm">
                Search Engines
            </div>
        </div>

        <div class="tab mt-4 flex flex-col items-center" v-bind:class="currentTab === SettingsTabs.THEME ? 'activeTab' : ''"
            @click="emit('click', SettingsTabs.THEME)">
            <div class="w-full flex justify-center">
                <BrushSVG class="h-7 w-7 fillAccent" />
            </div>
            <div class="w-full flex justify-center text-sm">
                Theming
            </div>
        </div>

        <div class="tab mt-4 flex flex-col items-center"
            v-bind:class="currentTab === SettingsTabs.EXTENSIONS ? 'activeTab' : ''"
            @click="emit('click', SettingsTabs.EXTENSIONS)">
            <div class="w-full flex justify-center">
                <PluginSVG class="h-7 w-7 fillAccent" />
            </div>
            <div class="w-full flex justify-center text-sm">
                Extensions
            </div>
        </div>

        <div class="tab mt-4 flex flex-col items-center" v-bind:class="currentTab === SettingsTabs.ABOUT ? 'activeTab' : ''"
            @click="emit('click', SettingsTabs.ABOUT)">
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