<script setup lang="ts">
import { onMounted, ref } from 'vue';
import ThreeDotsSVG from "../../assets/icons/three_dots_vertical.svg"
import { open, save } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api';
import { getSettings, getTheme } from './Settings';
import { emit, listen } from '@tauri-apps/api/event';
import PrimaryButton from '../../components/PrimaryButton.vue';
import { WebviewWindow } from '@tauri-apps/api/window';

const menu = ref();

const backgroundColor = ref("");
const currentBackgroundColor = ref("");

const secondaryBackgroundColor = ref("");
const currentSecondaryBackgroundColor = ref("");

const tertiaryBackgroundColor = ref("");
const currentTertiaryBackgroundColor = ref("");

const accentColor = ref("");
const currentAccentColor = ref("");

const onAccentColor = ref("");
const currentOnAccentColor = ref("");

const dangerColor = ref("");
const currentDangerColor = ref("");

const onDangerColor = ref("");
const currentOnDangerColor = ref("");

const textColor = ref("");
const currentTextColor = ref("");

const secondaryTextColor = ref("");
const currentSecondaryTextColor = ref("");

const updateThemeEmit = ref();

onMounted(() => {
    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    });
})

async function loadTheme() {
    let theme = await getTheme();

    backgroundColor.value = theme.background;
    currentBackgroundColor.value = theme.background;

    secondaryBackgroundColor.value = theme.secondary_background;
    currentSecondaryBackgroundColor.value = theme.secondary_background;

    tertiaryBackgroundColor.value = theme.tertiary_background;
    currentTertiaryBackgroundColor.value = theme.tertiary_background;

    accentColor.value = theme.accent;
    currentAccentColor.value = theme.accent;

    onAccentColor.value = theme.on_accent;
    currentOnAccentColor.value = theme.on_accent;

    dangerColor.value = theme.danger;
    currentDangerColor.value = theme.danger;

    onDangerColor.value = theme.on_danger;
    currentOnDangerColor.value = theme.on_danger;

    textColor.value = theme.text;
    currentTextColor.value = theme.text;

    secondaryTextColor.value = theme.secondary_text;
    currentSecondaryTextColor.value = theme.secondary_text;
}

async function saveTheme() {

    var settings = await getSettings();

    settings.theme.background = currentBackgroundColor.value;
    settings.theme.secondary_background = currentSecondaryBackgroundColor.value;
    settings.theme.tertiary_background = currentTertiaryBackgroundColor.value;
    settings.theme.accent = currentAccentColor.value;
    settings.theme.on_accent = currentOnAccentColor.value;
    settings.theme.danger = currentDangerColor.value;
    settings.theme.on_danger = currentOnDangerColor.value;
    settings.theme.text = currentTextColor.value;
    settings.theme.secondary_text = currentSecondaryTextColor.value;

    invoke("update_settings", { settings_json: JSON.stringify(settings) });
    emit("updateTheme");
    loadTheme();
}

function toggleIconMenu() {

    if (menu.value.style.display == "block") {
        menu.value.style.display = "none";
    } else {
        menu.value.style.display = "block";
    }
}

async function exportTheme() {

    closeMenu();

    let path = await save({
        defaultPath: "CustomTheme.yml",
        filters: [
            {
                name: "Yaml",
                extensions: ["yml"]
            }
        ]
    })

    if (path !== null) {
        invoke("export_theme", { path: path })
    }
}

async function importTheme() {

    closeMenu();

    let path = await open({
        filters: [
            {
                name: "Yaml",
                extensions: ["yml"]
            }
        ]
    })

    if (path !== null) {
        invoke("import_theme", { path: path });

        emit("updateTheme");

        loadTheme();
    }
}

function closeMenu() {
    menu.value.style.display = "none";
}

function isHexColor(v: string): boolean {
    return /^#([0-9a-f]{3}){1,2}$/i.test(v)
}

function isSaveButtonDisabled() {
    return !isHexColor(currentBackgroundColor.value)
        || !isHexColor(currentSecondaryBackgroundColor.value)
        || !isHexColor(currentTertiaryBackgroundColor.value)
        || !isHexColor(currentAccentColor.value)
        || !isHexColor(currentOnAccentColor.value)
        || !isHexColor(currentDangerColor.value)
        || !isHexColor(currentOnDangerColor.value)
        || !isHexColor(currentTextColor.value)
        || !isHexColor(currentSecondaryTextColor.value)
}

function openCommunityThemesDialog() {

    closeMenu();

    new WebviewWindow("communityThemesDialog", {
        width: 800,
        height: 800,
        transparent: true,
        center: true,
        url: "community-themes-dialog",
        title: "Community Themes"
    })
}

</script>

<template>
    <div class="p-4">
        <div class="flex items-start">
            <div class="text-2xl flex-grow ml-2">Theming</div>
            <div class="flex flex-grow justify-end">
                <button class="h-[40px] w-[40px] flex items-center justify-center menuButton" @click="toggleIconMenu()">
                    <ThreeDotsSVG class="h-5 w-5 menuButtonIcon" />
                </button>
                <div class="menu-content" ref="menu">
                    <button class="w-full p-2 flex justify-start hover:opacity-80 focus:opacity-80"
                        @click="exportTheme()">Export Theme</button>
                    <button class="w-full p-2 flex justify-start hover:opacity-80 focus:opacity-80"
                        @click="importTheme()">Import Theme</button>
                    <button class="w-full p-2 flex justify-start hover:opacity-80 focus:opacity-80"
                        @click="openCommunityThemesDialog()">
                        Community Themes
                    </button>
                </div>
            </div>
        </div>

        <div class="max-w-[700px] test">
            <div class="ml-3 mt-2 text-xl">Preview</div>
            <div class="previewBox mt-2 p-6 rounded-3xl w-fit">
                <div class="flex">
                    <button class=" previewButton p-2 pr-4 pl-4 rounded-full hover:opacity-80 w-full">Abc</button>
                    <button
                        class=" ml-4 previewDangerButton p-2 pr-4 pl-4 rounded-full hover:opacity-80 w-full">Abc</button>
                </div>
                <div class="p-4 previewCurrentSecondaryBackground rounded-3xl mt-4">
                    <div class="ml-3 previewText">Abc</div>
                    <input class="previewInput w-full" placeholder="Abc" />
                </div>
            </div>
            <div class="ml-2 mt-2 text-xl">Colors</div>
            <div class=" items-center secondaryBackground p-6 rounded-[28px]">
                <div class="flex items-center">
                    <div class="flex-grow">Background</div>
                    <input v-model="currentBackgroundColor" type="color" class="colorInput">
                </div>
            </div>
            <div class=" mt-1 items-center secondaryBackground p-6 rounded-[28px]">
                <div class="flex items-center">
                    <div class="flex-grow">Secondary Background</div>
                    <input v-model="currentSecondaryBackgroundColor" type="color" class="colorInput">
                </div>
            </div>
            <div class=" mt-1 items-center secondaryBackground p-6 rounded-[28px]">
                <div class="flex items-center">
                    <div class="flex-grow">Tertiary Background</div>
                    <input v-model="currentTertiaryBackgroundColor" type="color" class="colorInput">
                </div>
            </div>
            <div class=" mt-1 items-center secondaryBackground p-6 rounded-[28px]">
                <div class="flex items-center">
                    <div class="flex-grow">Accent</div>
                    <input v-model="currentAccentColor" type="color" class="colorInput">
                </div>
            </div>
            <div class=" mt-1 items-center secondaryBackground p-6 rounded-[28px]">
                <div class="flex items-center">
                    <div class="flex-grow">On Accent</div>
                    <input v-model="currentOnAccentColor" type="color" class="colorInput">
                </div>
            </div>
            <div class=" mt-1 items-center secondaryBackground p-6 rounded-[28px]">
                <div class="flex items-center">
                    <div class="flex-grow">Danger</div>
                    <input v-model="currentDangerColor" type="color" class="colorInput">
                </div>
            </div>
            <div class=" mt-1 items-center secondaryBackground p-6 rounded-[28px]">
                <div class="flex items-center">
                    <div class="flex-grow">On Danger</div>
                    <input v-model="currentOnDangerColor" type="color" class="colorInput">
                </div>
            </div>
            <div class=" mt-1 items-center secondaryBackground p-6 rounded-[28px]">
                <div class="flex items-center">
                    <div class="flex-grow">Text</div>
                    <input v-model="currentTextColor" type="color" class="colorInput">
                </div>
            </div>
            <div class=" mt-1 items-center secondaryBackground p-6 rounded-[28px]">
                <div class="flex items-center">
                    <div class="flex-grow">Secondary Text</div>
                    <input v-model="currentSecondaryTextColor" type="color" class="colorInput">
                </div>
            </div>
            <div class="flex mt-4">
                <PrimaryButton text="Save" @click="saveTheme()" :disabled="isSaveButtonDisabled()" :expand="true" />
            </div>
        </div>
    </div>
</template>

<style scoped>



.previewBox {
    background-color: v-bind(currentBackgroundColor);
    outline: 2px solid v-bind(currentSecondaryBackgroundColor);
}

.previewButton {
    background-color: v-bind(currentAccentColor);
    color: v-bind(currentOnAccentColor);
}

.previewDangerButton {
    background-color: v-bind(currentDangerColor);
    color: v-bind(currentOnDangerColor);
}

.previewColor {
    border: 1px solid v-bind(textColor);
    border-radius: 9999px;
}

.previewInput {
    background-color: v-bind(currentTertiaryBackgroundColor);
    padding: 8px;
    padding-right: 12px;
    padding-left: 12px;
    border-radius: 48px;
    color: v-bind(currentTextColor);
}

.previewInput::placeholder {
    color: v-bind(currentSecondaryTextColor);
}

.colorInput {
    appearance: none;
    -webkit-appearance: none;
    cursor: pointer;
    width: 80px;
    height: 40px;
    background-color: transparent;
}

.colorInput::-webkit-color-swatch {
    border-radius: 48px;
    border: 2px solid v-bind(textColor);
}

.previewCurrentBackground {
    background-color: v-bind(currentBackgroundColor);
}

.previewText {
    color: v-bind(currentTextColor);
}

.previewCurrentSecondaryBackground {
    background-color: v-bind(currentSecondaryBackgroundColor);
}

.previewCurrentTertiaryBackground {
    background-color: v-bind(currentTertiaryBackgroundColor);
}

.previewCurrentAccent {
    background-color: v-bind(currentAccentColor);
}

.previewCurrentOnAccent {
    background-color: v-bind(currentOnAccentColor);
}

.previewCurrentDanger {
    background-color: v-bind(currentDangerColor);
}

.previewCurrentOnDanger {
    background-color: v-bind(currentOnDangerColor);
}

.previewCurrentText {
    background-color: v-bind(currentTextColor);
}

.previewCurrentSecondaryText {
    background-color: v-bind(currentSecondaryTextColor);
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.tertiaryBackground {
    background-color: v-bind(tertiaryBackgroundColor);
}



.menuButton {
    background-color: v-bind(secondaryBackgroundColor);
    border-radius: 48px;
}

.menuButton:hover {
    outline: 2px solid v-bind(accentColor);
}

.menuButton:focus {
    outline: 2px solid v-bind(accentColor);
}

.menuButtonIcon {
    fill: v-bind(accentColor);
}

.menu-content {
    margin-top: 60px;
    margin-right: 20px;
    display: none;
    position: absolute;
    background-color: v-bind(tertiaryBackgroundColor);
    padding: 10px;
    z-index: 9999;
    right: 0;
    border-radius: 14px;
    outline: 2px solid v-bind(accentColor);
}
</style>