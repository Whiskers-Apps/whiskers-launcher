<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getRoundnessInPixels, getSettings, getTheme, updateSettings } from './Settings';

import SearchSVG from "../../assets/icons/search.svg"
import SettingsSVG from "../../assets/icons/settings.svg"
import Slider from '../../components/Slider.vue';
import Switch from '../../components/Switch.vue';
import { listen } from '@tauri-apps/api/event';


const showSearchIcon = ref(false);
const showSettingsIcon = ref(false);
const showPlaceholder = ref(false);
const borderRadius = ref();
const borderRadiusInput = ref();
const borderWidth = ref();
const borderWidthInput = ref();
const layout = ref();

const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");

const updateThemeEmit = ref();

onMounted(async () => {
    let settings = await getSettings()

    showSearchIcon.value = settings.search.show_search_icon;
    showSettingsIcon.value = settings.search.show_settings_icon;
    showPlaceholder.value = settings.search.show_placeholder;
    layout.value = settings.results.layout.type;

    borderRadius.value = `${settings.search.border_radius}px`;
    borderRadiusInput.value = settings.search.border_radius;
    borderWidth.value = `${settings.search.border_width}px`;
    borderWidthInput.value = settings.search.border_width;

    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    })
})

async function loadTheme() {
    let theme = await getTheme();

    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    accentColor.value = theme.accent;
}

function getSearchHeightClass(): string{
  switch(layout.value){
    case "Small": { return "smallHeight" }
    case "Medium": { return "mediumHeight" }
    default: { return "largeHeight" }
  }
}

function updateBorderRadius() {
    borderRadius.value = `${borderRadiusInput.value}px`;
    update()
}

function updateBorderWidth() {
    borderWidth.value = `${borderWidthInput.value}px`
    update()
}

async function update() {

    let settings = await getSettings();

    settings.search.border_width = borderWidthInput.value;
    settings.search.border_radius = borderRadiusInput.value;
    settings.search.show_search_icon = showSearchIcon.value;
    settings.search.show_settings_icon = showSettingsIcon.value;
    settings.search.show_placeholder = showPlaceholder.value;

    updateSettings(settings);
}

</script>

<template>
    <div class="max-w-[700px] p-4">

        <div class="text-2xl ml-2">Search</div>

        <div class="text-lg mt-1 ml-2">Preview</div>

        <div class="background flex items-center rounded  preview " :class="getSearchHeightClass()">
            <div v-if="showSearchIcon" class="">
                <SearchSVG class="w-5 h-5 stroke" />
            </div>
            <div class="flex-grow">
                <div v-if="showPlaceholder">Search</div>
            </div>
            <div v-if="showSettingsIcon" class="">
                <SettingsSVG class="w-5 h-5 stroke" />
            </div>
        </div>

        <div class="text-lg mt-2 ml-2">Settings</div>

        <div class="secondaryBackground p-6 rounded-[28px] flex mb-1">
            <div class="flex-grow">
                <div class=" font-semibold">Show Search Icon</div>
                <div>If enabled it will show the search icon</div>
            </div>
            <div>
                <Switch :checked="showSearchIcon" @update:checked="showSearchIcon = $event; update()" />
            </div>
        </div>

        <div class="secondaryBackground p-6 rounded-[28px] flex mb-1">
            <div class="flex-grow">
                <div class=" font-semibold">Show Settings Icon</div>
                <div>If enabled it will show the settings icon</div>
            </div>
            <div>
                <Switch :checked="showSettingsIcon" @update:checked="showSettingsIcon = $event; update()" />
            </div>
        </div>

        <div class="secondaryBackground p-6 rounded-[28px] flex mb-1">
            <div class="flex-grow">
                <div class=" font-semibold">Show Placeholder</div>
                <div>If enabled it will show the placeholder text</div>
            </div>
            <div>
                <Switch :checked="showPlaceholder" @update:checked="showPlaceholder = $event; update()" />
            </div>
        </div>

        <div class="secondaryBackground p-6 rounded-[28px] mb-1">

            <div class=" font-semibold">Border Radius ({{ borderRadius }})</div>
            <div>It changes the border radius of the search box.</div>
            <div class=" mt-2">
                <Slider :min="0" :max="48" :step="1" :value="borderRadiusInput"
                    @update:value="borderRadiusInput = $event; updateBorderRadius()" />
            </div>
        </div>

        <div class="secondaryBackground p-6 rounded-[28px] mb-1">
            <div class=" font-semibold">Border Width ({{ borderWidth }})</div>
            <div>It changes the border width of the search box. From 0px to 10px</div>
            <div class=" mt-2">
                <Slider :min="0" :max="10" :step="1" :value="borderWidthInput"
                    @update:value="borderWidthInput = $event; updateBorderWidth()" />
            </div>
        </div>
    </div>
</template>

<style scoped>

.border {
    border-color: v-bind(tertiaryBackgroundColor);
}

.preview {
    border-radius: v-bind(borderRadius);
    border: v-bind(borderWidth) solid v-bind(accentColor);
    padding-left: 16px;
    padding-right: 16px;
}

.secondaryBackgroundColor {
    background-color: v-bind(secondaryBackgroundColor);
}

input[type="range"]::-webkit-slider-runnable-track {
    background: v-bind(secondaryBackgroundColor);
    border-radius: 9999px;
    height: 1.2rem;
    border: 1.2px solid v-bind(accentColor);
}

input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    margin-top: -1px;
    background-color: v-bind(accentColor);
    height: 1.2rem;
    width: 1.2rem;
    border-radius: 9999px;
}

.stroke {
    fill: none;
    stroke: v-bind(accentColor);
    stroke-width: 2;
}

.smallHeight{
  min-height: 50px;
}

.mediumHeight{
  min-height: 60px;
}

.largeHeight{
  min-height: 70px;
}
</style>