<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getSettings, updateSettings } from '@pages/Settings/Settings';


import Slider from '@/components/Slider.vue';
import Switch from '@/components/Switch.vue';
import Select from '@/components/Select.vue';
import { SelectOption } from '@/components/Select.vue';
import { WebviewWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';

import SectionDivider from "@components/SectionDivider.vue";
import { invoke } from '@tauri-apps/api';
import { convertFileSrc } from '@tauri-apps/api/tauri';

import TrashSVG from "@icons/trash.svg";


interface AppIndex {
    icon_path: string,
    exec_path: string,
    name: string
}


const blacklist = ref<AppIndex[]>([]);
const blacklistListener = ref();

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

onMounted(async () => {

    let settings = await getSettings();

    resultsCount.value = settings.results.results_count;
    splitUI.value = settings.results.split_ui;
    layout.value = settings.results.layout;
    blacklist.value = await invoke("get_blacklist_apps");

    blacklistListener.value = listen("update-blacklist", async (_event) => {
        blacklist.value = await invoke("get_blacklist_apps");
    });
});

async function update() {

    let settings = await getSettings();
    settings.results.results_count = resultsCount.value;
    settings.results.split_ui = splitUI.value;
    settings.results.layout = layout.value;

    updateSettings(settings);
}

async function openAddToBlacklistDialog() {

    new WebviewWindow("add-to-blacklist-dialog", {
        resizable: false,
        center: true,
        url: "/add-to-blacklist-dialog",
        title: "Add To Blacklist",
        width: 800,
        height: 800
    });
}

</script>

<template>
    <div class=" p-4">
        <div class="ml-2 text-2xl">Results</div>

        <div class="section">
            <div class=" p-6">
                <div class=" font-semibold">Results Count ({{ resultsCount }})</div>
                <div>The amount of results to show</div>
                <div class="flex mt-2">
                    <Slider :min="2" :max="8" :step="1" :value="resultsCount"
                        @update:value="resultsCount = $event; update()" />
                </div>
            </div>

            <SectionDivider />

            <div class="flex p-6">
                <div class="flex-grow">
                    <div class=" font-semibold">Split UI</div>
                    <div>Split search bar and results</div>
                </div>
                <div class="flex ml-2">
                    <Switch :checked="splitUI" @update:checked="splitUI = $event; update();" />
                </div>
            </div>

            <SectionDivider />

            <div class="p-6">
                <div class=" font-semibold">Layout</div>
                <div>The density of the results layout.</div>
                <div class="flex mt-2" v-if="layout !== ''">
                    <Select :value="layout" :options="layoutOptions" @update-value="layout = $event.value; update();" />
                </div>
            </div>

            <SectionDivider />

            <div class="p-6">
                <div class="flex w-full items-center">
                    <div class=" flex-grow font-semibold">Blacklist</div>
                    <button class="tertiaryBackground p-2 pl-4 pr-4 rounded-full hover:opacity-90 focus:opacity-90"
                        @click="openAddToBlacklistDialog()">Add</button>
                </div>
                <div class="mt-4" v-if="blacklist.length === 0">Blacklist is empty. Click add to add a app to filter it on
                    app results</div>

                <div v-else class="mt-4"></div>

                <div v-for="(app, index) in blacklist" :key="`blacklist_app_${index}`" class="flex flex-col  ">
                    <div class="flex pb-4 pt-4 items-center">
                        <img :src="convertFileSrc(app.icon_path)" class="w-8 h-8">
                        <div class="ml-4 flex-grow">{{ app.name }}</div>
                        <button class="tertiaryBackground p-2 rounded-full hover:opacity-90"
                            @click="invoke('remove_from_blacklist', { path: app.exec_path })">
                            <TrashSVG class="w-5 h-5 strokeText" />
                        </button>
                    </div>
                    <SectionDivider v-if="index + 1 < blacklist.length" />
                </div>
            </div>
        </div>
    </div>
</template>

<style></style>