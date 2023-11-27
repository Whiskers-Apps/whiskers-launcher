<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { ExtensionSettings, getSettings, getTheme } from './Settings';
import { listen } from '@tauri-apps/api/event';
import { open as openLink } from "@tauri-apps/api/shell";
import lighttigerxivPFP from "@images/lighttigerxiv.jpeg";
import GitHubSVG from "@icons/github.svg";
import DiscordSVG from "@icons/discord.svg"

import SectionDivider from '@components/SectionDivider.vue';

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
        <div class="text-2xl ml-2">About</div>
        <div class="mt-2 ml-2 text-xl">App</div>
        <div class="section">
            <div class="p-4 items flex mt-2 ">
                <b>Version</b>
                <div class="flex-grow"></div>
                <div class="ml-4 oneLineText">0.2.1-alpha</div>
            </div>

            <SectionDivider />

            <div class="p-4 flex mt-1 ">
                <b>Extensions</b>
                <div class="flex-grow"></div>
                <div class="oneLineText">{{ extensions.length }}</div>
            </div>

            <SectionDivider />

            <div class="p-4 flex items-center mt-1 ">
                <b>Source</b>
                <div class="flex-grow"></div>
                <button @click="openLink('https://github.com/lighttigerXIV/simple-keyboard-launcher')">
                    <GitHubSVG class="h-[35px] w-[35px] fillIcon" />
                </button>
            </div>
        </div>
        <div class="mt-4 ml-3 text-xl">Developers</div>
        <div class="section">
            <div class="p-4 flex items-center">
                <img :src="lighttigerxivPFP" class="h-[50px] w-[50px] rounded-full profilePicture">
                <div class="ml-4 flex-grow">lighttigerXIV</div>
                <button @click="openLink('https://discord.com/users/598945126089228327')">
                    <DiscordSVG class="h-[35px] w-[35px] fillIcon" />
                </button>
                <button @click="openLink('https://github.com/lighttigerXIV')" class="ml-4">
                    <GitHubSVG class="h-[35px] w-[35px] fillIcon" />
                </button>
            </div>
        </div>
    </div>
</template>
<style>
.link {
    color: v-bind(accentColor);
}

.link:hover {
    text-decoration: underline;
}

.fillIcon {
    fill: v-bind(textColor);
}

.profilePicture{
    outline: 2px solid v-bind(tertiaryBackgroundColor);
}
</style>