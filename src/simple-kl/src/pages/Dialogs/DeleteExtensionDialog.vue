<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '../Settings/Settings';
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';
import { useRoute } from 'vue-router';
import DangerButton from '../../components/DangerButton.vue';
import SecondaryButton from '../../components/SecondaryButton.vue';

const route = useRoute();
const extensionID = ref(route.query.id ?? '')
const backgroundColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");
const textColor = ref("");
const secondaryTextColor = ref("");

const cloningExtension = ref(false);

onMounted(async () => {
    let theme = await getTheme();
    backgroundColor.value = theme.background;
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    accentColor.value = theme.accent;
    textColor.value = theme.text;
});

async function deleteExtension() {

    cloningExtension.value = true;

    await invoke("delete_extension", { id: extensionID.value });

    cloningExtension.value = false;

    emit("updateExtensions");

    invoke("close_window");
}

</script>
<template>
    <div class="h-screen w-screen main flex flex-col justify-center">
        Are you sure you want to delete the extension?
        <div class="flex mt-4">
            <SecondaryButton class="mr-4" text="Cancel" :expand="true" @click="invoke('close_window')" />
            <DangerButton text="Delete" :expand="true" @click="deleteExtension()" />
        </div>
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