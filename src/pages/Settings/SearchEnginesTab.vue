<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { SearchEngineSettings, getSettings, getTheme, updateSettings } from './Settings';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { hexToCSSFilter } from "hex-to-css-filter"
import EditSVG from "../../assets/icons/edit.svg"
import ThreeDotsSVG from "../../assets/icons/three_dots_vertical.svg"
import { WebviewWindow } from "@tauri-apps/api/window"
import CheckSVG from "../../assets/icons/check-circle.svg"
import { emit, listen } from '@tauri-apps/api/event';

const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");
const onAccentColor = ref("");
const textColor = ref("");


const searchEngines = ref<SearchEngineSettings[]>();
const addButtonHovering = ref(false);

const updateSettingsListener = ref();


onMounted(async () => {
    let settings = await getSettings();
    searchEngines.value = settings.search_engines;

    loadTheme()

    updateSettingsListener.value = await listen("updateSettings", async (_event) => {
        let settings = await getSettings();
        searchEngines.value = settings.search_engines;
    })
})

async function loadTheme() {
    let theme = await getTheme();
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    accentColor.value = theme.accent;
    onAccentColor.value = theme.on_accent;
    textColor.value = theme.text;
}

function getIconFilter(tintIcon: boolean): string {


    if (tintIcon) {
        let filter = hexToCSSFilter(accentColor.value);
        return filter.filter.replace(";", "")

    } else {
        return "none"
    }


    return "none"
}

function openEditDialog(index: number) {

    closeMenus();

    new WebviewWindow("edit-search-engine", {
        url: `edit-search-engine/${index}`,
        title: "Edit Search Engine",
        resizable: false,
        width: 700,
        height: 670,
        center: true,
        transparent: true
    })
}

function openDeleteDialog(index: number) {

    closeMenus();

    new WebviewWindow("delete-search-engine", {
        url: `delete-search-engine?index=${index}`,
        title: "Delete Search Engine",
        resizable: false,
        width: 800,
        height: 200,
        center: true,
        transparent: true
    })
}

function openAddDialog() {

    closeMenus();

    new WebviewWindow("add-search-engine", {
        url: "add-search-engine",
        title: "Add Search Engine",
        resizable: false,
        width: 700,
        height: 670,
        center: true,
        transparent: true
    })
}

function toggleMenu(index: number) {
    let e = document.getElementById(`menu-${index}`)!!;

    closeMenus(index);

    if (e.style.display == "block") {
        closeMenus();
    } else {
        e.style.display = "block";
    }
}

function closeMenus(exception?: number) {

    searchEngines.value?.forEach((_searchEngine, index) => {
        if (index !== exception) {
            document.getElementById(`menu-${index}`)!!.style.display = "none";
        }
    });
}

async function makeDefault(index: number) {
    let settings = await getSettings();
    let newSearchEngines: SearchEngineSettings[] = [];

    settings.search_engines.forEach((searchEngine, se_index) => {

        let newEngine = searchEngine;
        newEngine.default = se_index === index ? true : false;
        newSearchEngines.push(newEngine);
    })

    settings.search_engines = newSearchEngines;
    updateSettings(settings);
    emit("updateSettings");
    closeMenus();
}

document.addEventListener('keydown', (event) => {
    if (event.key === "Escape") {
        closeMenus();
    }
})

</script>

<template>
    <div class="max-w-[700px] p-4">
        <div class="flex items-start">
            <div class="text-2xl flex-grow ml-2">Search Engines</div>
            <button @mouseover="addButtonHovering = true" @mouseout="addButtonHovering = false"
                class="flex items-center addButton " @click="openAddDialog()">
                Add
            </button>
        </div>

        <div class="mt-4"></div>

        <div class="p-6 rounded-[28px] mb-1  flex secondaryBackground items-center"
            v-for="(searchEngine, index) in searchEngines">

            <div class="p-1 rounded-lg">
                <img class="h-[25px] w-[25px]" :src="convertFileSrc(searchEngine.icon ?? '')"
                    :style="{ filter: getIconFilter(searchEngine.tint_icon) }">
            </div>

            <div class="ml-4 p-1 rounded-lg whitespace-nowrap text-ellipsis text-lg">{{ searchEngine.name }}</div>

            <div v-if="searchEngine.default" class="ml-2">
                <CheckSVG class="h-[25px] w-[25px] accentStroke stroke-2" />
            </div>

            <div class="flex-grow"></div>

            <div class="accentText font-bold ml-4  tertiaryBackground pt-1 pb-1 pr-2 pl-2  rounded-md">{{
                searchEngine.keyword }}</div>

            <button class="icon p-2 ml-4 rounded-lg" @click="openEditDialog(index)">
                <EditSVG class="fillAccent w-5 h-5" />
            </button>

            <div class="menu">
                <button class="icon p-2 ml-4 rounded-lg" @click="toggleMenu(index)">
                    <ThreeDotsSVG class="h-5 w-4 fillAccent" />
                </button>
                <div class="menu-content p-2 rounded-xl" :id="`menu-${index}`">
                    <div class="menuButton flex items-center">
                        <button class="text-start w-full whitespace-nowrap p-2 hover:opacity-80 focus:opacity-80"
                            @click="makeDefault(index)">Make Default</button>
                    </div>
                    <div class="menuButton flex items-center">
                        <button class="text-start w-full p-2 hover:opacity-80 focus:opacity-80"
                            @click="openDeleteDialog(index)">Delete</button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.accentText {
    color: v-bind(accentColor);
}

.onAccentFill {
    color: v-bind(onAccentColor);
}

.accentBackground {
    background-color: v-bind(accentColor);
}

.fillAccent {
    fill: v-bind(accentColor);
}

.accentStroke {
    fill: none;
    stroke: v-bind(accentColor);
}

.border {
    border: 2px solid v-bind(accentColor);
}

.icon {
    border: 2px solid v-bind(secondaryBackgroundColor);
}

.icon:hover {
    background-color: v-bind(tertiaryBackgroundColor);
    border-color: v-bind(accentColor);
}

.addButton {
    border: 2px solid v-bind(accentColor);
    border-radius: 48px;
    padding: 6px;
    padding-left: 24px;
    padding-right: 24px;
    outline: none;
}

.addButtonIcon {
    fill: v-bind(accentColor);

}

.addButton:focus {
    border-radius: 20px;
}

.menu {
    position: relative;
    display: inline-block;
}

.menu-content {
    display: none;
    position: absolute;
    background-color: v-bind(tertiaryBackgroundColor);
    z-index: 9999;
    right: 0;
    border: 2px solid v-bind(accentColor);
    
}

.stroke {
    fill: none;
    stroke: v-bind(textColor);
    stroke-width: 2
}
</style>