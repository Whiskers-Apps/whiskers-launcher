<script setup lang="ts">

import { onMounted, ref } from "vue";
import { Settings, getSettings, SearchEngine, updateSettings } from "../Settings/Settings"
import { useRoute } from "vue-router";
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { open } from "@tauri-apps/api/dialog"
import { emit } from "@tauri-apps/api/event"
import { hexToCSSFilter } from "hex-to-css-filter";
import { invoke } from "@tauri-apps/api";
import Switch from "../../components/Switch.vue"

const searchEngine = ref<SearchEngine>();
const index = +useRoute().params.index;
const backgroundColor = ref("");
const accentColor = ref("");
const onAccentColor = ref("")
const textColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const iconFilter = ref("");

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
    backgroundColor.value = settings.theming.background;
    accentColor.value = settings.theming.accent;
    onAccentColor.value = settings.theming.on_accent;
    textColor.value = settings.theming.text;
    secondaryBackgroundColor.value = settings.theming.secondary_background;
    tertiaryBackgroundColor.value = settings.theming.tertiary_background;


    searchEngine.value = settings.search_engines[index]
    tintIcon.value = searchEngine.value.tint_icon;
    realIconPath.value = searchEngine.value.icon;
    iconPath.value = convertFileSrc(searchEngine.value.icon ?? '' + "?" + Math.floor(Math.random() * 1000));
    keyword.value = searchEngine.value.keyword;
    name.value = searchEngine.value.name;
    query.value = searchEngine.value.query;

    applyIconFilter();
})

</script>

<template>
    <div class="background  h-[500px] p-2 text ">

        <div class="flex h-[100px] mt-2 items-end">
            <button class="icon p-2 rounded-lg" @click="selectIcon()">
                <img class="h-[80px] w-[80px] object-contain tintIcon" :src="iconPath">
            </button>

            <div class="ml-2 secondaryBackground p-2 rounded-lg flex items-center">
                Tint
                <Switch
                    :background-color="backgroundColor"
                    :tertiary-background-color="tertiaryBackgroundColor"
                    :accent-color="accentColor"
                    :checked="tintIcon"
                    @update:checked="toggleTintIcon($event)"
                />
            </div>
        </div>

        <div class=" items-center secondaryBackground p-2 rounded-lg mt-2">
            <div>Keyword</div>
            <div class="flex">
                <input class="flex-grow tertiaryBackground p-1 rounded-lg outline-none" :value="keyword"
                    @input="keyword = ($event.target as HTMLInputElement).value">
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-2 rounded-lg">
            <div>Name</div>
            <div class="flex">
                <input class="flex-grow tertiaryBackground p-1 rounded-lg outline-none" :value="name"
                    @input="name = ($event.target as HTMLInputElement).value">
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-2 rounded-lg">
            <div>Query</div>
            <div class="flex">
                <input class="flex-grow tertiaryBackground p-1 rounded-lg outline-none" :value="query"
                    @input="query = ($event.target as HTMLInputElement).value">
            </div>
        </div>

        <div class="flex">
            <button class="p-2 mt-4 flex-grow button rounded-lg hover:rounded-2xl" @click="save()">
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

.button {
    background-color: v-bind(accentColor);
    color: v-bind(onAccentColor);
}

.icon {
    border: 1px solid v-bind(backgroundColor);
    background-color: v-bind(secondaryBackgroundColor);
}

.icon:hover {
    background: v-bind(tertiaryBackgroundColor);
    border-color: v-bind(accentColor);
}

.tintIcon {
    filter: v-bind(iconFilter);
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.tertiaryBackground {
    background-color: v-bind(tertiaryBackgroundColor);
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
</style>