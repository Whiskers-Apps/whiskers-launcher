<script setup lang="ts">

import { onMounted, ref } from "vue";
import { Settings, getSettings, SearchEngineSettings, updateSettings, getTheme } from "../Settings/Settings"
import { useRoute } from "vue-router";
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { open } from "@tauri-apps/api/dialog"
import { emit } from "@tauri-apps/api/event"
import { hexToCSSFilter } from "hex-to-css-filter";
import { invoke } from "@tauri-apps/api";
import Switch from "../../components/Switch.vue"
import ThreeDotsSVG from "../../assets/icons/three_dots_vertical.svg"

const searchEngine = ref<SearchEngineSettings>();
const index = +useRoute().params.index;
const backgroundColor = ref("");
const accentColor = ref("");
const onAccentColor = ref("")
const textColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const iconFilter = ref("");
const iconMenu = ref();


const iconPath = ref("");
const realIconPath = ref()
const tintIcon = ref(false);
const keyword = ref("");
const name = ref("");
const query = ref("");


async function selectIcon() {
    const selected = await open({
        multiple: false,
        filters: [{
            name: "Images (png, jpg, jpeg, svg)",
            extensions: ["png", "jpg", "jpeg", "svg"]
        }]
    })

    if (selected !== null) {
        realIconPath.value = selected.toString();
        iconPath.value = convertFileSrc(selected.toString() ?? '' + "?" + Math.floor(Math.random() * 1000))
    }
}


function applyIconFilter() {

    if (tintIcon.value) {
        iconFilter.value = hexToCSSFilter(accentColor.value).filter.replace(";", "");
    } else {
        iconFilter.value = "none"
    }
}

function toggleTintIcon(checked: boolean) {
    tintIcon.value = checked;
    applyIconFilter();
}

async function save() {

    let settings = await getSettings();

    settings.search_engines[index].icon = realIconPath.value;
    settings.search_engines[index].tint_icon = tintIcon.value;
    settings.search_engines[index].keyword = keyword.value;
    settings.search_engines[index].name = name.value;
    settings.search_engines[index].query = query.value;

    updateSettings(settings);
    emit("updateSettings");
    invoke("close_window");
}

onMounted(async () => {

    let settings: Settings = await getSettings();
    let theme = await getTheme();
    backgroundColor.value = theme.background;
    accentColor.value = theme.accent;
    onAccentColor.value = theme.on_accent;
    textColor.value = theme.text;
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;


    searchEngine.value = settings.search_engines[index]
    tintIcon.value = searchEngine.value.tint_icon;
    realIconPath.value = searchEngine.value.icon;
    iconPath.value = convertFileSrc(searchEngine.value.icon ?? '' + "?" + Math.floor(Math.random() * 1000));
    keyword.value = searchEngine.value.keyword;
    name.value = searchEngine.value.name;
    query.value = searchEngine.value.query;

    applyIconFilter();
})

function clearIcon() {
    realIconPath.value = "";
    iconPath.value = "";
    tintIcon.value = false;
    iconMenu.value.style.display = "none"
}

function toggleIconMenu() {

    if (iconMenu.value.style.display == "block") {
        iconMenu.value.style.display = "none";
    } else {
        iconMenu.value.style.display = "block";
    }
}

function isSaveButtonDisabled(): boolean {
    return keyword.value.trim() === "" || name.value.trim() === "" || query.value.trim() === ""
}

document.addEventListener('keydown', (event) => {
    if (event.key === "Escape") {
        iconMenu.value.style.display = "none"
    }
})

</script>

<template>
    <div class="background  h-[670px] p-4 text ">

        <div class="flex rounded-2xl items-start">
            <button class="icon p-2 rounded-2xl" @click="selectIcon()">
                <div v-if="iconPath !== ''">
                    <img class="h-[150px] w-[150px] object-contain tintIcon" :src="iconPath">
                </div>
                <div v-else>
                    <div class="h-[150px] w-[150px]"></div>
                </div>
            </button>

            <div class="menu flex flex-grow justify-end">
                <button class="h-[50px] w-[50px] flex items-center justify-center threeDotsButton"
                    @click="toggleIconMenu()">
                    <ThreeDotsSVG class="h-5 w-5 threeDotsIcon" />
                </button>
                <div class="menu-content" ref="iconMenu">
                    <div class=" rounded-2xl p-2 flex items-center">
                        <div class="flex-grow">Tint</div>
                        <Switch :background-color="backgroundColor" :tertiary-background-color="tertiaryBackgroundColor"
                            :accent-color="accentColor" :checked="tintIcon" @update:checked="toggleTintIcon($event)" />
                    </div>
                    <button
                        class=" tertiaryBackground hover:opacity-80 focus:opacity-80 p-2 rounded-2xl flex items-start w-full "
                        @click="clearIcon()">
                        Clear
                    </button>
                </div>
            </div>
        </div>

        <div class=" items-center secondaryBackground p-4 rounded-2xl mt-2">
            <div class="ml-2 mb-1">Keyword</div>
            <div class="flex">
                <input class="flex-grow tertiaryBackground p-2 pl-4 pr-4 rounded-full outline-none input" :value="keyword"
                    @input="keyword = ($event.target as HTMLInputElement).value" :maxlength="10">
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-4 rounded-2xl">
            <div class="ml-2 mb-1">Name</div>
            <div class="flex">
                <input class="flex-grow tertiaryBackground p-2 pl-4 pr-4 rounded-full outline-none input" :value="name"
                    @input="name = ($event.target as HTMLInputElement).value">
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-4 rounded-2xl">
            <div class="ml-2 mb-1">Query</div>
            <div class="flex">
                <input class="flex-grow tertiaryBackground p-2 pl-4 pr-4 rounded-full outline-none input" :value="query"
                    @input="query = ($event.target as HTMLInputElement).value">
            </div>
        </div>

        <div class="flex">
            <button
                class="p-2 mt-4 flex-grow button rounded-full focus:rounded-xl focus:enabled:opacity-80 hover:enabled:opacity-80"
                @click="save()" :disabled="isSaveButtonDisabled()">
                Save
            </button>
        </div>
    </div>
</template>

<style scoped>
.text {
    color: v-bind(textColor);
}

.background {
    background-color: v-bind(backgroundColor);
}

.button:enabled {
    background-color: v-bind(accentColor);
    color: v-bind(onAccentColor);
}


.input:focus {
    outline: 2px solid v-bind(accentColor);
}

.button:disabled {
    background-color: v-bind(tertiaryBackgroundColor);
    color: v-bind(textColor);
    outline: v-bind(secondaryBackgroundColor);
}

.clearButton:hover {
    opacity: 0.8;
}

.icon {
    border: 2px solid v-bind(backgroundColor);
    background-color: v-bind(tertiaryBackgroundColor);
}

.icon:hover {
    background: v-bind(tertiaryBackgroundColor);
    border-color: v-bind(accentColor);
}

.tintIcon {
    filter: v-bind(iconFilter);
}

.threeDotsButton {
    background-color: v-bind(secondaryBackgroundColor);
    border-radius: 48px;
}

.threeDotsButton:hover {
    outline: 2px solid v-bind(accentColor);
}

.threeDotsButton:focus {
    outline: 2px solid v-bind(accentColor);
}

.threeDotsIcon {
    fill: v-bind(accentColor);
}

.divider {
    background-color: v-bind(accentColor);
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.tertiaryBackground {
    background-color: v-bind(tertiaryBackgroundColor);
}


.menu-content {
    margin-top: 60px;
    margin-right: 20px;
    display: none;
    position: absolute;
    background-color: v-bind(tertiaryBackgroundColor);
    padding: 10px;
    min-width: 250px;
    z-index: 9999;
    right: 0;
    border-radius: 14px;
    outline: 2px solid v-bind(accentColor);
}</style>