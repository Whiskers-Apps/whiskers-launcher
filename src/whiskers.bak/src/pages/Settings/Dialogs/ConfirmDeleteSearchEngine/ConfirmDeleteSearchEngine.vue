<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { Settings } from "@pages/Settings/ViewModel";
import { invoke } from "@tauri-apps/api/tauri";
import SecondaryButton from "@/components/SecondaryButton.vue";
import PrimaryButton from "@/components/PrimaryButton.vue";
import { appWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";
import { DeleteSearchEnginePayload } from "@/DialogPayloads";


const id = +(useRoute().query.id ?? -1);
const settings = ref<Settings>();
const hasLoaded = ref(false);

const backgroundMain = ref("");
const accentPrimary = ref("");
const textOnBackground = ref("");

onMounted(async () => {
    settings.value = await invoke("get_settings");

    backgroundMain.value = settings.value!!.theme.background_main;
    accentPrimary.value = settings.value!!.theme.accent_primary;
    textOnBackground.value = settings.value!!.theme.text_on_background;

    hasLoaded.value = true;
});

function deleteSearchEngine(){

    let payload: DeleteSearchEnginePayload = { id: id};

    emit("delete-search-engine", payload);
    appWindow.close();
}

</script>

<template>
    <div v-if="hasLoaded" class="main p-4 h-screen max-h-screen flex flex-col justify-center">
        <div class="title">Delete Search Engine</div>
        <div>Are you sure you want to delete the search engine?</div>
        <div class="mt-2 flex justify-end">
            <SecondaryButton text="Cancel" :theme="settings!!.theme" @click="appWindow.close()"/>
            <PrimaryButton class="ml-2" text="Delete" :danger="true" :theme="settings!!.theme" @click="deleteSearchEngine()"/>
        </div>
    </div>
</template>

<style scoped>
.main {
    background-color: v-bind(backgroundMain);
    color: v-bind(textOnBackground);
}

.title {
    color: v-bind(accentPrimary);
    font-size: 24px;
    font-weight: 700;
}
</style>
