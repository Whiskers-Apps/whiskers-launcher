<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Settings } from "@pages/Settings/ViewModel";
import { getHexCssFilter, getSettings } from "@/utils";
import SwitchSetting from "@/components/SwitchSetting.vue";
import InputSetting from "@/components/InputSetting.vue";
import PrimaryButton from "@/components/PrimaryButton.vue";
import SecondaryButton from "@/components/SecondaryButton.vue";
import { open } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { SearchEnginePayload } from "@/DialogPayloads";
import { emit } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { useRoute } from "vue-router";

const id = +(useRoute().query.id ?? -1);
const route = useRoute();

class UiState {
    hasLoaded = false;
    settings: Settings | null = null;
    realIconPath: string | null = null;
    iconPath: string | null = null;
    tintIcon: boolean = false;
    keyword: string = "";
    name: string = "";
    query: string = "";
    default: boolean = false;
}

const uiState = ref<UiState>(new UiState());
const backgroundMain = ref("");
const backgroundSecondary = ref("");
const backgroundTertiary = ref("");
const textOnBackground = ref("");

onMounted(async () => {
    uiState.value.settings = await getSettings();

    backgroundMain.value = uiState.value.settings.theme.background_main;
    backgroundSecondary.value = uiState.value.settings.theme.background_secondary;
    backgroundTertiary.value = uiState.value.settings.theme.background_tertiary;
    textOnBackground.value = uiState.value.settings.theme.text_on_background;

 
    const searchEngine = uiState.value.settings.search_engines.filter(se => se.id === id)[0];
    
    
    uiState.value.tintIcon = searchEngine.tint_icon;
    uiState.value.realIconPath = searchEngine.icon_path;
    uiState.value.iconPath = searchEngine.icon_path !== null ? convertFileSrc(searchEngine.icon_path) : null;
    uiState.value.keyword = searchEngine.keyword;
    uiState.value.name = searchEngine.name;
    uiState.value.query = searchEngine.query;
    uiState.value.default = searchEngine.default;

    uiState.value.hasLoaded = true;
    
});

function isSaveButtonDisabled(): boolean {
    return (
        uiState.value.keyword.trim().length === 0 ||
        uiState.value.name.trim().length === 0 ||
        uiState.value.query.trim().length == 0
    );
}

async function clearIcon() {
    uiState.value.realIconPath = null;
    uiState.value.iconPath = null;
}

async function selectIcon() {
    const selected = await open({
        multiple: false,
        filters: [
            {
                name: "Images (png, jpg, jpeg, svg)",
                extensions: ["png", "jpg", "jpeg", "svg"],
            },
        ],
    });

    if (selected !== null) {
        uiState.value.realIconPath = selected.toString();
        uiState.value.iconPath = convertFileSrc(
            `${selected.toString()}?${Math.floor(Math.random() * 1000)}`,
        );
    }
}

function saveSearchEngine() {
    let payload: SearchEnginePayload = {
        id: id,
        icon_path: uiState.value.realIconPath,
        tint_icon: uiState.value.tintIcon,
        keyword: uiState.value.keyword,
        query: uiState.value.query,
        name: uiState.value.name,
        default: uiState.value.default,
    };

    emit("edit-search-engine", payload);

    appWindow.close();
}
</script>

<template>
    <div v-if="uiState.hasLoaded" class="main h-screen max-h-screen p-4 flex flex-col">
        <div class="flex-grow overflow-auto">
            <div class="flex justify-center">
                <div class=" flex flex-col items-center">
                    <button class="icon-box aspect-square rounded-2xl scale-100" @click="selectIcon()">
                        <img v-if="uiState.iconPath !== null && !uiState.tintIcon" class="h-full w-full rounded-2xl"
                            :src="uiState.iconPath" />

                        <img v-if="uiState.iconPath !== null && uiState.tintIcon"
                            class="h-full w-full rounded-2xl scale-100" :src="uiState.iconPath" :style="{
                                filter: getHexCssFilter(
                                    uiState.settings!!.theme.accent_primary,
                                ),
                            }" />
                    </button>

                    <SecondaryButton class="mt-2" v-if="uiState.realIconPath !== null " text="Clear Icon"
                        :theme="uiState.settings!!.theme" @click="clearIcon()" />
                </div>
            </div>

            <SwitchSetting title="Tint Icon" description="If enabled, it tints the search engine icon"
                :checked="uiState.tintIcon" :theme="uiState.settings!!.theme" @update-checked="uiState.tintIcon = $event" />

            <InputSetting class="mt-2" title="Keyword" description="The search engine keyword" placeholder="Keyword"
                :value="uiState.keyword" :theme="uiState.settings!!.theme" @update-value="uiState.keyword = $event" />
            <InputSetting class="mt-2" title="Name" description="The search engine name" placeholder="Name"
                :value="uiState.name" :theme="uiState.settings!!.theme" @update-value="uiState.name = $event" />
            <InputSetting class="mt-2" title="Query" description="The search engine query. Tip: Use %s as the search text"
                placeholder="Query" :value="uiState.query" :theme="uiState.settings!!.theme"
                @update-value="uiState.query = $event" />
        </div>
        <div class="flex justify-end mt-2">
            <PrimaryButton text="Save" :theme="uiState.settings!!.theme" :disabled="isSaveButtonDisabled()"
                @click="saveSearchEngine()" />
        </div>
    </div>
</template>

<style scoped>
.main {
    background-color: v-bind(backgroundMain);
    color: v-bind(textOnBackground);
}

.icon-box {
    background-color: v-bind(backgroundSecondary);
    width: 200px;
    height: 200px;
}

.icon-box:hover {
    background-color: v-bind(backgroundTertiary);
    cursor: pointer;
}
</style>
