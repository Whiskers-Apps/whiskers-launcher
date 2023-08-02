<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { SearchEngine, getSettings, getTheme, updateSettings } from '../Settings/Settings';
import { useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api';


const backgroundColor = ref("");
const textColor = ref("");
const accentColor = ref("");
const onAccentColor = ref("");
const dangerColor = ref("");
const onDangerColor = ref("");

const index = +useRoute().query.index!!;
const searchEngine = ref<SearchEngine>();


onMounted(async () => {
    let settings = await getSettings();
    let theme = await getTheme();
    backgroundColor.value = theme.background;
    textColor.value = theme.text;
    accentColor.value = theme.accent;
    onAccentColor.value = theme.on_accent;
    dangerColor.value = theme.danger;
    onDangerColor.value = theme.on_danger;

    searchEngine.value = settings.search_engines[index];
})

async function deleteSearchEngine(){
    let settings = await getSettings();

    settings.search_engines = settings.search_engines.filter((_engine, engineIndex)=> engineIndex !== index);
    updateSettings(settings);
    invoke('close_window');
}
</script>
<template>
    <div class="h-[200px] background text p-4">
        <div class="text-2xl">Delete Search Engine</div>
        <div>Are you sure you want to delete <b>{{ searchEngine?.name }}</b>?</div>
        <div class="flex mt-4">
            <button class="p-2 flex-grow cancelButton" @click="invoke('close_window')">Cancel</button>
            <div class="w-[20px]"></div>
            <button class="p-2 flex-grow deleteButton" @click="deleteSearchEngine()">Delete</button>
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

.cancelButton{
    border-radius: 48px;
    border: 2px solid v-bind(accentColor);
}

.cancelButton:hover{
    background-color: v-bind(accentColor);
    color: v-bind(onAccentColor);
}

.cancelButton:focus{
    background-color: v-bind(accentColor);
    border-radius: 16px;
    color: v-bind(onAccentColor);
    outline: none;
}

.deleteButton{
    background-color: v-bind(dangerColor);
    color: v-bind(onDangerColor);
    border-radius: 48px;
}

.deleteButton:focus{
    border-radius: 16px;
    opacity: 0.8;
    outline: none;
}

.deleteButton:hover{
    opacity: 0.8;
}


</style>