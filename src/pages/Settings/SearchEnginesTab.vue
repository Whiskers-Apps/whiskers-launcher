<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { SearchEngine, getSettings } from './Settings';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { hexToCSSFilter } from "hex-to-css-filter"
import EditSVG from "../../assets/icons/edit.svg"
import ThreeDotsSVG from "../../assets/icons/three_dots_vertical.svg"
import { listen } from '@tauri-apps/api/event';
import { WebviewWindow } from "@tauri-apps/api/window"
import { event } from '@tauri-apps/api';


const searchEngines = ref<SearchEngine[]>();
const updateSettingsEvent = ref();
const addButtonHovering = ref(false);

const props = defineProps({
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
    },
    textColor: {
        required: true,
        type: String
    },
    onAccentColor: {
        required: true,
        type: String
    }
})

onMounted(async () => {
    let settings = await getSettings();
    searchEngines.value = settings.search_engines;

    updateSettingsEvent.value = await listen("updateSettings", async (_event) => {
        let settings = await getSettings();
        searchEngines.value = settings.search_engines;
    })
})

function getIconFilter(tintIcon: boolean): string {

    if (tintIcon) {
        let filter = hexToCSSFilter(props.accentColor);
        return filter.filter.replace(";", "")

    } else {
        return "none"
    }
}

function openEditDialog(index: number) {

    new WebviewWindow("edit-search-engine", {
        url: `edit-search-engine/${index}`,
        title: "Edit Search Engine",
        resizable: false,
        width: 700,
        height: 500,
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

document.addEventListener('keydown', (event) => {
    if (event.key === "Escape") {
        closeMenus();
    }
})

</script>

<template>
    <div class="overflow-x-clip">
        <div class="flex items-start">
            <div class="text-3xl flex-grow ml-3">Search Engines</div>
            <button @mouseover="addButtonHovering = true" @mouseout="addButtonHovering = false"
                class="flex items-center addButton p-2 hover:rounded-2xl">
                Add
            </button>
        </div>

        <div class="mt-4"></div>

        <div class="p-4 mb-1 rounded-3xl flex secondaryBackground items-center"
            v-for="(searchEngine, index) in searchEngines">

            <div class="p-1 rounded-lg">
                <img class="h-[30px] w-[30px]" :src="convertFileSrc(searchEngine.icon ?? '')"
                    :style="{ filter: getIconFilter(searchEngine.tint_icon) }">
            </div>

            <div class="ml-4 p-1 rounded-lg whitespace-nowrap text-ellipsis text-lg">{{ searchEngine.name }}</div>

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
                <div class="menu-content rounded-xl" :id="`menu-${index}`">
                    <div class="menuButton flex items-center">
                        <button class="text-start w-full p-2">Make Default</button>
                    </div>
                    <div class="menuButton flex items-center">
                        <button class="text-start w-full p-2" @click="openDeleteDialog(index)">Delete</button>
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

.menuButton {
    width: 250px;
    border: 2px solid transparent;
}

.menuButton:hover {
    background-color: v-bind(backgroundColor);
    border: 2px solid v-bind(accentColor);
    border-radius: 8px;
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.tertiaryBackground {
    background-color: v-bind(tertiaryBackgroundColor);
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
    padding: 10px;
    padding-left: 30px;
    padding-right: 30px;
    outline: none;
}

.addButtonIcon {
    fill: v-bind(accentColor);

}

.addButton:focus {
    border-radius: 16px;
}

.menu {
    position: relative;
    display: inline-block;
}

.menu-content {
    display: none;
    position: absolute;
    background-color: v-bind(tertiaryBackgroundColor);
    padding: 10px;
    min-width: 250px;
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