<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { ExtensionSettings, getSettings, getTheme } from './Settings';
import { listen } from '@tauri-apps/api/event';
import { open as openLink } from "@tauri-apps/api/shell";
import lighttigerxivPFP from "../../assets/images/lighttigerxiv.jpg";
import GitHubSVG from "../../assets/icons/github.svg";

const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");
const textColor = ref("");
const updateThemeEmit = ref();
const extensions = ref<ExtensionSettings[]>([]);

onMounted(async () => {

    let settings = await getSettings();
    extensions.value = settings.extensions;

    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    });
})

async function loadTheme() {
    let theme = await getTheme();
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    accentColor.value = theme.accent;
    textColor.value = theme.text;
}

</script>
<template>
    <div class="p-4">
        <div class="text-3xl ml-3">About</div>
        <div class="mt-4 ml-3 text-xl">App</div>
        <div class="card flex mt-2">
            <b>Version:</b>
            <div class="flex-grow"></div>
            <div class="ml-4 oneLineText">Alpha-0.0.1</div>
        </div>

        <div class="card flex mt-1">
            <b>Extensions:</b>
            <div class="flex-grow"></div>
            <div class="oneLineText">{{ extensions.length }}</div>
        </div>

        <div class="card flex mt-1">
            <b>Source:</b>
            <div class="flex-grow"></div>
            <button @click="openLink('https://github.com/lighttigerXIV/simple-keyboard-launcher')">
                <div class="oneLineText link">https://github.com/lighttigerXIV/simple-keyboard-launcher</div>
            </button>
        </div>
        <div class="mt-4 ml-3 text-xl">Developers</div>
        <div class="card flex items-center">
            <img :src="lighttigerxivPFP" class="h-[40px] w-[40px] rounded-full">
            <div class="ml-4 flex-grow">lighttigerXIV</div>
            <button @click="openLink('https://github.com/lighttigerXIV')">
                <GitHubSVG class="h-[40px] w-[40px] fillIcon" />
            </button>
        </div>
    </div>
</template>
<style>
.card {
    background-color: v-bind(secondaryBackgroundColor);
    padding: 16px;
    border-radius: 24px;
}

.link {
    color: v-bind(accentColor);
}

.link:hover {
    text-decoration: underline;
}

.fillIcon {
    fill: v-bind(textColor);
}
</style>