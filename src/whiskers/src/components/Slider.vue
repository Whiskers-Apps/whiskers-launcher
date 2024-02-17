<script setup lang="ts">
import { Theme } from '@/pages/Settings/ViewModel';
import { listen } from '@tauri-apps/api/event';
import { PropType, onMounted, ref } from 'vue';

const props = defineProps<{
    min: number,
    max: number,
    step: number,
    value: number,
    theme: Theme
}>();

const backgroundTertiary = ref(props.theme.background_tertiary);
const accentPrimary = ref(props.theme.accent_primary);

const emit = defineEmits([
    "update:value"
])

onMounted(async ()=>{
    await listen("load-theme", (_event)=>{
        backgroundTertiary.value = props.theme.background_tertiary;
        accentPrimary.value = props.theme.accent_primary;
    });
});
</script>

<template>
    <div class=" flex-grow flex overflow-hidden rounded-full">
        <input type="range" class="flex-grow p-1" :step="step" :min="min" :max="max" :value="value"
            @input="emit('update:value', +($event.target as HTMLInputElement).value)">
    </div>
</template>

<style scoped>

input[type="range"] {
  -webkit-appearance: none;
  appearance: none;
  background: transparent;
  cursor: pointer;
  width: 15rem;
}

input[type="range"]:focus {
  outline: none;
}

input[type="range"] {
    margin: auto;
    -webkit-appearance: none;
    appearance: none;
    position: relative;
    overflow: hidden;
    height: 30px;
    width: 200px;
    cursor: pointer;
    border-radius: 0;
}

::-webkit-slider-runnable-track {
    background: v-bind(backgroundTertiary);
    border-radius: 999px;
}

::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 30px;
    height: 30px;
    background: v-bind(backgroundTertiary);
    box-shadow: -2000px 0 0 1985px v-bind(accentPrimary);
    border: 4px solid v-bind(accentPrimary);
    border-radius: 999px;
}


::-ms-fill-lower {
    background: v-bind(accentPrimary);
}

::-ms-ticks-after {
    display: none;
}

::-ms-ticks-before {
    display: none;
}

::-ms-track {
    background: v-bind(backgroundTertiary);
    color: transparent;
    height: 40px;
    border: none;
}

::-ms-tooltip {
    display: none;
}
</style>