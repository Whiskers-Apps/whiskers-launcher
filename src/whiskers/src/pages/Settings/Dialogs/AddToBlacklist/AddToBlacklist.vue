<script setup lang="ts">
import { getSettings, getHexCssFilter, getIconUrl } from "@/utils";
import { ref, onMounted } from "vue";
import { Settings } from "@pages/Settings/ViewModel";
import { ViewModel } from "./ViewModel";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import PrimaryButton from "@components/PrimaryButton.vue";

const vm = ref<ViewModel>(new ViewModel());
const settings = ref<Settings>();
const backgroundMain = ref("");
const backgroundTertiary = ref("");
const accentPrimary = ref("");
const textOnBackground = ref("");

onMounted(async () => {
    settings.value = await getSettings();
    backgroundMain.value = settings.value.theme.background_main;
    backgroundTertiary.value = settings.value.theme.background_tertiary;
    accentPrimary.value = settings.value.theme.accent_primary;
    textOnBackground.value = settings.value.theme.text_on_background;

    await vm.value.load();
});
</script>

<template>
    <div class="main h-screen max-h-screen flex flex-col p-4" v-if="vm.hasLoaded ? vm.hasLoaded : false">
        <div class="title">Add To Blacklist</div>
        <div>Click on the apps you would like to blacklist.</div>
        <div class="flex justify-center mt-2 mb-2">
            <div class="divider"></div>
        </div>
        <div class="flex-grow overflow-auto">
            <div class="cursor-pointer" v-for="app in vm.currentWhitelistedApps" :key="app.exec_path">
                <div class="flex items-center p-2 app-card rounded-2xl" @click="vm.toggleApp(app)">
                    <img width="40" class="mr-2" :src="app.icon_path
                            ? convertFileSrc(app.icon_path)
                            : getIconUrl('default-app-icon.svg')
                    " :style="{filter: app.icon_path ? 'none' : getHexCssFilter(accentPrimary) }" />
                    <div class="font-medium flex-grow">{{ app.name }}</div>
                    <div v-if="app.checked" class="ml-2">
                        <img :src="getIconUrl('check.svg')" :style="{ filter: getHexCssFilter(accentPrimary) }"
                            width="24" />
                    </div>
                </div>
            </div>
        </div>
        <div class="mt-2 flex items-center">
            <div class="flex-grow text-start mr-2">
                {{ vm.checkedCount }} app(s) selected
            </div>
            <PrimaryButton text="Add" :theme="settings!!.theme" :disabled="vm.checkedCount === 0"
                @click="vm.addToBlacklist()" />
        </div>
    </div>
</template>

<style scoped>
.main {
    color: v-bind(textOnBackground);
    background-color: v-bind(backgroundMain);
}

.title {
    color: v-bind(accentPrimary);
    font-size: 24px;
    font-weight: 700;
}

.divider {
    width: 100%;
    height: 5px;
    border-radius: 48px;
    background-color: v-bind(accentPrimary);
}

.app-card:hover {
    background-color: v-bind(backgroundTertiary);
}
</style>
