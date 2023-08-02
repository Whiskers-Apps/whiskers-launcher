<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '../pages/Settings/Settings';
import { listen } from '@tauri-apps/api/event';


const accentColor = ref("");
const onAccentColor = ref("");
const tertiaryBackgroundColor = ref("");
const updateThemeEmit = ref();

const emit = defineEmits([
    "click"
])

const props = defineProps({
    expand: {
        type: Boolean
    },
    disabled: {
        type: Boolean
    },
    text: {
        required: true,
        type: String
    }
})

onMounted(() => {
    loadTheme();

    updateThemeEmit.value = listen("updateTheme", (_event) => {
        loadTheme();
    })
})

async function loadTheme() {
    let theme = await getTheme();
    accentColor.value = theme.accent;
    onAccentColor.value = theme.on_accent;
    tertiaryBackgroundColor.value = theme.tertiary_background;
}
</script>

<template>
    <div class="w-full">
        <button class="primaryButton p-2 rounded-full" :class="props.expand ? 'w-full' : ''" @click="emit('click')" :disabled="disabled">{{ text }}</button>
    </div>
</template>

<style scoped>
.primaryButton {
    background-color: v-bind(accentColor);
    color: v-bind(onAccentColor);
    border: 1px solid v-bind(tertiaryBackgroundColor);
    padding-left: 16px;
    padding-right: 16px;
}

.primaryButton:hover:enabled {
    opacity: 0.8;
}

.primaryButton:focus:enabled {
    opacity: 0.8;
    border-radius: 16px;
    outline: none;
}

.primaryButton:disabled{
    background-color: v-bind(tertiaryBackgroundColor);
    color: v-bind(text);
}
</style>