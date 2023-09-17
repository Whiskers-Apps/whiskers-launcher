<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getRoundnessInPixels, getSettings, getTheme, updateSettings } from './Settings';

import SearchSVG from "../../assets/icons/search.svg"
import SettingsSVG from "../../assets/icons/settings.svg"
import Slider from '../../components/Slider.vue';
import Switch from '../../components/Switch.vue';
import { listen } from '@tauri-apps/api/event';


const showSearchIcon = ref(false);
const showSettingsIcon = ref(true);
const roundness = ref();
const roundnessInput = ref();
const borderWidth = ref();
const borderWidthInput = ref()

const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");

const updateThemeEmit = ref();

onMounted(async () => {
    let settings = await getSettings()

    showSearchIcon.value = settings.search_box.show_search_icon;
    showSettingsIcon.value = settings.search_box.show_settings_icon;
    roundness.value = getRoundnessInPixels(settings.search_box.roundness);
    roundnessInput.value = settings.search_box.roundness;
    borderWidth.value = `${settings.search_box.border_width}px`;
    borderWidthInput.value = settings.search_box.border_width;

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

function updateRoundness() {
    roundness.value = getRoundnessInPixels(roundnessInput.value);
    update()
}

function updateBorderWidth() {
    borderWidth.value = `${borderWidthInput.value}px`
    update()
}

async function update() {

    let settings = await getSettings();

    settings.search_box.border_width = borderWidthInput.value;
    settings.search_box.roundness = roundnessInput.value;
    settings.search_box.show_search_icon = showSearchIcon.value;
    settings.search_box.show_settings_icon = showSettingsIcon.value;

    updateSettings(settings);
}

</script>

<template>
    <div class="max-w-[700px] p-4">

        <div class="text-2xl ml-2">Search Box</div>

        <div class="text-lg mt-1 ml-2">Preview</div>

        <div class="background flex items-center rounded  preview ">
            <div v-if="showSearchIcon" class="mr-2">
                <SearchSVG class="w-5 h-5 stroke" />
            </div>
            <div class="flex-grow">
                Search
            </div>
            <div v-if="showSettingsIcon" class="ml-2">
                <SettingsSVG class="w-5 h-5 stroke" />
            </div>
        </div>

        <div class="text-lg mt-2 ml-2">Settings</div>

        <div class="secondaryBackground p-6 rounded-[28px] flex mb-1">
            <div class="flex-grow">
                <div class=" font-semibold">Show Search Icon</div>
                <div>If enabled it will show the search icon</div>
            </div>
            <div class="flex items-center">
                <Switch :checked="showSearchIcon" @update:checked="showSearchIcon = $event; update()" />
            </div>
        </div>

        <div class="secondaryBackground p-6 rounded-[28px] flex mb-1">
            <div class="flex-grow">
                <div class=" font-semibold">Show Settings Icon</div>
                <div>If enabled it will show the settings icon</div>
            </div>
            <div class="flex items-center">
                <Switch :checked="showSettingsIcon" @update:checked="showSettingsIcon = $event; update()" />
            </div>
        </div>

        <div class="secondaryBackground p-6 rounded-[28px] mb-1">

            <div class=" font-semibold">Roundness</div>
            <div>It changes the roundness of the search box. From no round to fully round</div>
            <div class=" mt-2">
                <Slider :min="0" :max="9" :step="1" :value="roundnessInput"
                    @update:value="roundnessInput = $event; updateRoundness()" />
            </div>
        </div>

        <div class="secondaryBackground p-6 rounded-[28px] mb-1">

            <div class=" font-semibold">Border Width</div>
            <div>It changes the border width of the search box. From 0px to 6px</div>
            <div class="flex mt-2">
                <Slider :min="0" :max="6" :step="1" :value="borderWidthInput"
                    @update:value="borderWidthInput = $event; updateBorderWidth()" />
            </div>
        </div>
    </div>
</template>

<style scoped>
.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.border {
    border-color: v-bind(tertiaryBackgroundColor);
}

.preview {
    border-radius: v-bind(roundness);
    border: v-bind(borderWidth) solid v-bind(accentColor);
    height: 55px;
    padding: 12px;
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
</style>