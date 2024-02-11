<script setup lang="ts">
import { PropType } from "vue";
import Slider from "./Slider.vue";
import { Theme } from "@/pages/Settings/ViewModel";

const emit = defineEmits(["updateValue"]);

const props = defineProps({
    title: {
        required: true,
        type: String,
    },
    description: {
        required: true,
        type: String,
    },
    value: {
        required: true,
        type: Number,
    },
    min: {
        required: true,
        type: Number,
    },
    max: {
        required: true,
        type: Number,
    },
    step: {
        required: true,
        type: Number,
    },
    theme: {
        required: true,
        type: Object as PropType<Theme>,
    },
});
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
