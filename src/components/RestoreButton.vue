<script setup lang="ts">
import { onMounted, ref } from 'vue';
import RestoreSVG from "../assets/icons/restore.svg";
import { getTheme } from '../pages/Settings/Settings';
import { listen } from '@tauri-apps/api/event';

const emit = defineEmits(["click"]);
const tertiaryBackgroundColor = ref("");
const textColor = ref("");
const updateThemeEmit = ref();

onMounted(async () => {
    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    });
})

async function loadTheme() {
    let theme = await getTheme();
    tertiaryBackgroundColor.value = theme.tertiary_background;
    textColor.value = theme.text;
}

</script>
<template>
    <div>
        <button class="restoreButton" @click="emit('click')">
            <RestoreSVG class="restoreButtonIcon" />
        </button>
    </div>
</template>
<style scoped>
.restoreButton {
    background-color: v-bind(tertiaryBackgroundColor);
    padding: 12px;
    border-radius: 48px;
}

.restoreButton:hover{
    opacity: 0.8;
}

.restoreButtonIcon {
    height: 22px;
    width: 22px;
    fill: v-bind(textColor);
}
</style>