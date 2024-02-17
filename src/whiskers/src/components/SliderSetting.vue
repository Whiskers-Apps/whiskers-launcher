<script setup lang="ts">
import { PropType } from "vue";
import Slider from "./Slider.vue";
import { Theme } from "@/pages/Settings/ViewModel";

const emit = defineEmits(["updateValue"]);

const props = defineProps<{
    title: string,
    description: string,
    value: number,
    min: number,
    max: number,
    step: number,
    theme: Theme
}>();
</script>

<template>
    <div class="mr-2 ml-2">
        <div>
            <div class="title">{{ props.title }}</div>
            <div>{{ props.description }}</div>
        </div>

        <Slider class="mt-2" :min="props.min" :max="props.max" :step="props.step" :value="props.value" :theme="props.theme"
            @update:value="emit('updateValue', $event)" />
    </div>
</template>
