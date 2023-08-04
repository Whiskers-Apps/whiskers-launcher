<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { SettingsCategory, getTheme } from "./Settings"
import GeneralTab from './GeneralTab.vue';
import ThemesTab from './ThemesTab.vue';
import SearchBoxTab from './SearchBoxTab.vue'
import SearchEnginesTab from './SearchEnginesTab.vue';
import ExtensionsTab from './ExtensionsTab.vue';
import { listen } from '@tauri-apps/api/event';
import SearchSVG from "../../assets/icons/search.svg"
import HomeSVG from "../../assets/icons/home.svg"
import BrushSVG from "../../assets/icons/brush.svg"
import PluginSVG from "../../assets/icons/plugin.svg"
import InfoSVG from "../../assets/icons/info.svg"
import AboutTab from './AboutTab.vue';

const currentCategory = ref(SettingsCategory.GENERAL)

const backgroundColor = ref("")
const secondaryBackgroundColor = ref("")
const tertiaryBackgroundColor = ref("")
const accentColor = ref("")
const onAccentColor = ref("")
const dangerColor = ref("")
const onDangerColor = ref("")
const textColor = ref("")
const secondaryTextColor = ref("")

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
    secondaryTextColor.value = theme.seconday_text;
}


</script>

<template>
    <div class=" h-screen w-screen max-h-screen max-w-screen background flex text">
        <div class=" w-[300px] h-screen secondaryBackground p-2 overflow-auto">
            <div class="tab flex items-center"
                v-bind:class="currentCategory === SettingsCategory.GENERAL ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.GENERAL">
                <HomeSVG class="mr-3 h-7 w-7 strokeAccent" />
                General
            </div>
            <div class="tab mt-1 flex items-center"
                v-bind:class="currentCategory === SettingsCategory.SEARCH_BOX ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.SEARCH_BOX">
                <SearchSVG class="mr-3 h-7 w-7 strokeAccent" />
                Search Box
            </div>

            <div class="tab mt-1 flex items-center"
                v-bind:class="currentCategory === SettingsCategory.SEARCH_ENGINES ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.SEARCH_ENGINES">
                <SearchSVG class="mr-3 h-7 w-7 strokeAccent" />
                Search Engines
            </div>

            <div class="tab mt-1 flex items-center"
                v-bind:class="currentCategory === SettingsCategory.THEME ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.THEME">
                <BrushSVG class="mr-3 h-7 w-7 fillAccent" />
                Theming
            </div>

            <div class="tab mt-1 flex items-center"
                v-bind:class="currentCategory === SettingsCategory.EXTENSIONS ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.EXTENSIONS">
                <PluginSVG class="mr-3 h-7 w-7 fillAccent" />
                Extensions
            </div>

            <div class="tab mt-1 flex items-center"
                v-bind:class="currentCategory === SettingsCategory.ABOUT ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.ABOUT">
                <InfoSVG class="mr-3 h-7 w-7 fillAccent" />
                About
            </div>
        </div>

        <div class="flex-grow h-screen max-h-screen overflow-x-scroll">
            <div v-if="currentCategory === SettingsCategory.GENERAL">
                <GeneralTab :background-color="backgroundColor" :secondary-background-color="secondaryBackgroundColor"
                    :accent-color="accentColor" :text-color="textColor"
                    :tertiary-background-color="tertiaryBackgroundColor" />
            </div>

            <div v-if="currentCategory === SettingsCategory.SEARCH_BOX">
                <SearchBoxTab :background-color="backgroundColor" :secondary-background-color="secondaryBackgroundColor"
                    :tertiary-background-color="tertiaryBackgroundColor" :accent-color="accentColor" />
            </div>

            <div v-if="currentCategory === SettingsCategory.THEME">
                <ThemesTab />
            </div>
            <div v-if="currentCategory === SettingsCategory.SEARCH_ENGINES">
                <SearchEnginesTab :background-color="backgroundColor" :secondary-background-color="secondaryBackgroundColor"
                    :tertiary-background-color="tertiaryBackgroundColor" :accent-color="accentColor" :text-color="textColor"
                    :on-accent-color="onAccentColor" />
            </div>
            <div v-if="currentCategory === SettingsCategory.EXTENSIONS">
                <ExtensionsTab :background-color="backgroundColor" :secondary-background-color="secondaryBackgroundColor"
                    :tertiary-background-color="tertiaryBackgroundColor" :accent-color="accentColor"
                    :text-color="textColor" />
            </div>

            <div v-if="currentCategory === SettingsCategory.ABOUT">
                <AboutTab/>
            </div>
        </div>
    </div>
</template>

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

.activeTab {
    background-color: v-bind(tertiaryBackgroundColor);
    font-weight: 600;

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