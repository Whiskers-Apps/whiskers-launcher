<script setup lang="ts">
import Switch from "./Switch.vue";
import { Theme } from "@/pages/Settings/ViewModel";
import { ref } from "vue";

const emit = defineEmits(["updateChecked"]);

const props = defineProps<{
    title: string;
    description: string;
    checked: boolean;
    theme: Theme;
}>();

const accentPrimary = ref(props.theme.accent_primary);

</script>

<template>
    <div class="flex mr-2">
        <div class="flex flex-col flex-grow ml-2">
            <div class="title">{{ props.title }}</div>
            <div>{{ props.description }}</div>
        </div>
        <div class="ml-2 mt-2 flex items-start">
            <Switch :checked="props.checked" :theme="props.theme" @update:checked="emit('updateChecked', $event)" />
        </div>
    </div>
</template>

<style scoped>
.title {
    color: v-bind(accentPrimary);
    font-size: 24px;
    font-weight: 700;
}
</style>
