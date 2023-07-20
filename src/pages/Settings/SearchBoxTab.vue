<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { getRoundnessInPixels, getSettings, updateSetting } from './Settings';

import SearchSVG from "../../assets/icons/search.svg"
import SettingsSVG from "../../assets/icons/settings.svg"
import Slider from '../../components/Slider.vue';
import Switch from '../../components/Switch.vue';

defineProps({
    backgroundColor: {
        required: true,
        type: String
    },
    secondaryBackgroundColor: {
        required: true,
        type: String
    },
    tertiaryBackgroundColor: {
        required: true,
        type: String
    },
    accentColor: {
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
                <SearchSVG class="w-5 h-5 stroke" />
            </div>
            <div class="flex-grow">
                Search
            </div>
            <div v-if="showSettingsIcon" class="ml-2">
                <SettingsSVG class="w-5 h-5 stroke" />
            </div>
        </div>

        <div class="text-lg font-bold mt-2">Settings</div>

        <div class="secondaryBackground p-4 rounded-xl flex">
            <div class="flex-grow">
                <div class=" font-semibold">Show Search Icon</div>
                <div class="text-sm">If enabled it will show a search icon on the left</div>
            </div>
            <div class="flex items-center">
                <Switch :checked="showSearchIcon"
                    @update:checked="showSearchIcon = $event; updateSetting('search_box_show_search_icon', showSearchIcon)" />
            </div>
        </div>

        <div class="secondaryBackground p-4 rounded-xl flex mt-2">
            <div class="flex-grow">
                <div class=" font-semibold">Show Settings Icon</div>
                <div class="text-sm">If enabled it will show a settings icon on the right</div>
            </div>
            <div class="flex items-center">
                <Switch :checked="showSettingsIcon"
                    @update:checked="showSettingsIcon = $event; updateSetting('search_box_show_settings_icon', showSettingsIcon)" />
            </div>
        </div>

        <div class="secondaryBackground p-4 rounded-xl mt-2">

            <div class=" font-semibold">Roundness</div>
            <div class="text-sm">It changes the roundness of the search box. From no round to fully round</div>
            <div class=" mt-2">
                <Slider :min="0" :max="9" :step="1" :value="roundnessInput"
                    @update:value="roundnessInput = $event; updateSetting('search_box_roundness', +roundnessInput)" />
            </div>
        </div>

        <div class="secondaryBackground p-4 rounded-xl mt-2">

            <div class=" font-semibold">Border Width</div>
            <div class="text-sm">It changes the border width of the search box. From 0px to 6px</div>
            <div class="flex mt-2">
                <Slider
                    :min="0"
                    :max="6"
                    :step="1"
                    :value="borderWidthInput"
                    @update:value="borderWidthInput = $event; updateSetting('search_box_border_width', +borderWidthInput)"
                />
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