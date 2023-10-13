<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getSettings, updateSettings } from '@pages/Settings/Settings';


import Slider from '@/components/Slider.vue';
import Switch from '@/components/Switch.vue';
import Select from '@/components/Select.vue';
import ChevronDownSVG from "@icons/chevron-down.svg";
import ChevronUpSVG from "@icons/chevron-up.svg";

import { SelectOption } from '@/components/Select.vue';

const resultsCount = ref(0);
const splitUI = ref(false);
const layout = ref("");
const layoutOptions: SelectOption[] = [
    {
        value: "Small",
        text: "Small"
    },
    {
        value: "Medium",
        text: "Medium"
    },
    {
        value: "Large",
        text: "Large"
    }
]

const showBlacklist = ref(false)

onMounted(async () => {

    let settings = await getSettings();

    resultsCount.value = settings.results.results_count;
    splitUI.value = settings.results.split_ui;
    layout.value = settings.results.layout;
});

async function update() {

    let settings = await getSettings();
    settings.results.results_count = resultsCount.value;
    settings.results.split_ui = splitUI.value;
    settings.results.layout = layout.value;

    updateSettings(settings);
}

</script>

<template>
    <div class="max-w-[700px] p-4">
        <div class="ml-2 text-2xl">Results</div>

        <div class="secondaryBackground p-6 rounded-[28px] mb-1 mt-2">
            <div class=" font-semibold">Results Count ({{ resultsCount }})</div>
            <div>The amount of results to show</div>
            <div class="flex mt-2">
                <Slider :min="2" :max="8" :step="1" :value="resultsCount" @update:value="resultsCount = $event; update()" />
            </div>
        </div>

        <div class="secondaryBackground flex p-6 rounded-[28px] mb-1 mt-1">
            <div class="flex-grow">
                <div class=" font-semibold">Split UI</div>
                <div>Split search bar and results</div>
            </div>
            <div class="flex ml-2">
                <Switch :checked="splitUI" @update:checked="splitUI = $event; update();" />
            </div>
        </div>

        <div class="secondaryBackground p-6 rounded-[28px] mb-1 mt-1">
            <div class=" font-semibold">Layout</div>
            <div>The density of the results layout.</div>
            <div class="flex mt-2" v-if="layout !== ''">
                <Select :value="layout" :options="layoutOptions" @update-value="layout = $event; update();" />
            </div>
        </div>

        <div class="secondaryBackground p-6 rounded-[28px] mb-1 mt-1">
            <div class="flex w-full items-center" @click="showBlacklist = !showBlacklist">
                <div class=" flex-grow font-semibold">Blacklist</div>
                <ChevronDownSVG v-if="!showBlacklist" class="h-3 w-3 fillText" />
                <ChevronUpSVG v-if="showBlacklist" class="h-3 w-3 strokeText"/>
            </div>
        </div>
    </div>
</template>
