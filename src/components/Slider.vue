<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getSettings } from '../pages/Settings/Settings';


const props = defineProps({
    min: {
        required: true,
        type: Number
    },
    max: {
        required: true,
        type: Number
    },
    step: {
        required: true,
        type: Number
    },
    value: {
        required: true,
        type: Number
    }
})

const backgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");

const emit = defineEmits([
    "update:value"
])

onMounted(async ()=>{
    let settings = await getSettings();
    backgroundColor.value = settings.theming.background;
    tertiaryBackgroundColor.value = settings.theming.tertiary_background;
    accentColor.value = settings.theming.accent;
})

</script>

<template>
    <div class=" flex-grow flex overflow-hidden rounded-[999px]">
        <input type="range" class="flex-grow p-1" :step="step" :min="min" :max="max" :value="value"
            @input="emit('update:value', +($event.target as HTMLInputElement).value)">
    </div>
</template>

<style scoped>
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
    background: v-bind(tertiaryBackgroundColor);
    border-radius: 999px;
}

::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 30px;
    height: 30px;
    background: v-bind(tertiaryBackgroundColor);
    box-shadow: -2000px 0 0 1985px v-bind(accentColor);
    border: 4px solid v-bind(accentColor);
    border-radius: 999px;
}


::-ms-fill-lower {
    background: v-bind(accentColor);
}



::-ms-ticks-after {
    display: none;
}

::-ms-ticks-before {
    display: none;
}

::-ms-track {
    background: v-bind(tertiaryBackgroundColor);
    color: transparent;
    height: 40px;
    border: none;
}

::-ms-tooltip {
    display: none;
}
</style>