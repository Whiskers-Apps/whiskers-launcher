<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { getRoundnessInPixels, getSettings, updateSetting } from './Settings';

import SearchSVG from "../../assets/icons/search.svg"
import SettingsSVG from "../../assets/icons/settings.svg"


defineProps({
    backgroundColor:{
        required: true,
        type: String
    },
    secondaryBackgroundColor:{
        required: true,
        type: String
    },
    tertiaryBackgroundColor:{
        required: true,
        type: String
    },
    accentColor:{
        required: true,
        type: String
    }
})


const showSearchIcon = ref(false);
const showSettingsIcon = ref(true);
const roundness = ref();
const roundnessInput = ref();
const borderWidth = ref();
const borderWidthInput = ref()

onMounted(async () => {
    let settings = await getSettings()

    showSearchIcon.value = settings.search_box.show_search_icon;
    showSettingsIcon.value = settings.search_box.show_settings_icon;
    roundness.value = getRoundnessInPixels(settings.search_box.roundness);
    roundnessInput.value = settings.search_box.roundness;
    borderWidth.value = `${settings.search_box.border_width}px`;
    borderWidthInput.value = settings.search_box.border_width;
})

watch(roundnessInput, async (_, __) => {

    roundness.value = getRoundnessInPixels(roundnessInput.value);
})

watch(borderWidthInput, async (_, __) => {

    borderWidth.value = borderWidthInput.value + "px"
})



</script>

<template>
    <div>

        <div class="text-lg font-bold">Preview</div>

        <div class=" background flex items-center rounded pt-2 pb-2 pl-4 pr-4 preview ">
            <div v-if="showSearchIcon" class="mr-2">
                <SearchSVG class="w-5 h-5 stroke"/>
            </div>
            <div class="flex-grow">
                Search
            </div>
            <div v-if="showSettingsIcon" class="ml-2">
                <SettingsSVG class="w-5 h-5 stroke"/>
            </div>
        </div>

        <div class="text-lg font-bold mt-2">Settings</div>

        <div class="secondaryBackground p-4 rounded-xl border flex">
            <div class="flex-grow">
                <div class=" font-semibold">Show Search Icon</div>
                <div class="text-sm">If enabled it will show a search icon on the left</div>
            </div>
            <div class="flex items-center">
                <label class="switch">
                    <input type="checkbox" :checked="showSearchIcon" @input="showSearchIcon = ($event.target as HTMLInputElement).checked; updateSetting('search_box_show_search_icon', showSearchIcon)">
                    <span class="slider round"></span>
                </label>
            </div>
        </div>

        <div class="secondaryBackground p-4 rounded-xl border flex mt-2">
            <div class="flex-grow">
                <div class=" font-semibold">Show Settings Icon</div>
                <div class="text-sm">If enabled it will show a settings icon on the right</div>
            </div>
            <div class="flex items-center">
                <label class="switch">
                    <input type="checkbox" :checked="showSettingsIcon" @input="showSettingsIcon = ($event.target as HTMLInputElement).checked; updateSetting('search_box_show_settings_icon', showSettingsIcon)">
                    <span class="slider round"></span>
                </label>
            </div>
        </div>

        <div class="secondaryBackground p-4 rounded-xl border mt-2">

            <div class=" font-semibold">Roundness</div>
            <div class="text-sm">It changes the roundness of the search box. From no round to fully round</div>
            <div class="flex mt-2">
                <input type="range" class="flex-grow" step="1" min="0" max="9" :value="roundnessInput"
                    @input="roundnessInput = ($event.target as HTMLInputElement).value; updateSetting('search_box_roundness', +roundnessInput)">
            </div>
        </div>

        <div class="secondaryBackground p-4 rounded-xl border mt-2">

            <div class=" font-semibold">Border Width</div>
            <div class="text-sm">It changes the border width of the search box. From 0px to 6px</div>
            <div class="flex mt-2">
                <input type="range" class="flex-grow" step="1" min="0" max="6" :value="borderWidthInput"
                    @input="borderWidthInput = ($event.target as HTMLInputElement).value; updateSetting('search_box_border_width', +borderWidthInput)">
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

.preview{
    border-radius: v-bind(roundness);
    border: v-bind(borderWidth) solid v-bind(accentColor);
    height: 70px;
}


.switch {
    position: relative;
    display: inline-block;
    width: 60px;
    height: 30px;
    border: 4px solid v-bind(accentColor);
    border-radius: 9999px;
}

.switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

.slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: v-bind(backgroundColor);
}

.slider:before {
    position: absolute;
    content: "";
    height: 20px;
    width: 20px;
    left: 4px;
    bottom: 1px;
    background-color: v-bind(tertiaryBackgroundColor);
    border: 1px solid v-bind(accentColor);
}


input:checked+.slider {
    background-color: v-bind(accentColor);
}

input:focus+.slider {
    box-shadow: 0 0 1px v-bind(accentColor);
}

input:checked+.slider:before {
    -webkit-transform: translateX(26px);
    -ms-transform: translateX(26px);
    transform: translateX(26px);
}

/* Rounded sliders */
.slider.round {
    border-radius: 34px;
}

.slider.round:before {
    border-radius: 50%;
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

.stroke{
  fill: none;
  stroke: v-bind(accentColor);
  stroke-width: 2;
}
</style>