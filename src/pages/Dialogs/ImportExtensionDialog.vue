<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '../Settings/Settings';
import PrimaryButton from '../../components/PrimaryButton.vue';
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';


const backgroundColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");
const textColor = ref("");
const secondaryTextColor = ref("");

const urlInput = ref("");
const cloningExtension = ref(false);

onMounted(async () => {
    let theme = await getTheme();
    backgroundColor.value = theme.background;
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    accentColor.value = theme.accent;
    textColor.value = theme.text;
});

async function importExtension() {

    cloningExtension.value = true;

    await invoke("import_extension", { url: urlInput.value });

    cloningExtension.value = false;

    emit("updateExtensions");

    invoke("close_window");
}

</script>
<template>
    <div class="h-screen w-screen main">
        <div class="secondaryBackground rounded-3xl p-4">
            <div class="ml-3">Url</div>
            <input class="input" v-model="urlInput" placeholder="Extension url" />
        </div>
        <PrimaryButton class="mt-4" :expand="true" text="Clone" :disabled="urlInput.trim() === '' || cloningExtension"
            @click="importExtension()" />
    </div>
</template>
<style scoped>
.main {
    background-color: v-bind(backgroundColor);
    padding: 16px;
    color: v-bind(textColor);
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.input {
    width: 100%;
    border-radius: 48px;
    padding: 8px;
    padding-left: 16px;
    padding-right: 16px;
    background-color: v-bind(tertiaryBackgroundColor);
}

.input::placeholder {
    color: v-bind(secondaryTextColor);
}
</style>