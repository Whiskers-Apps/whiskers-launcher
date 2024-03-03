<script setup lang="ts">
import { Theme } from "@/pages/Settings/ViewModel";
import { onMounted, ref } from "vue";
import TextArea from "./TextArea.vue";
import { listen } from "@tauri-apps/api/event";

const emit = defineEmits(["updateValue"]);

const props = defineProps<{
    title: string;
    description: string;
    value: string;
    placeholder: string | null;
    theme: Theme;
}>();

const accentPrimary = ref(props.theme.accent_primary);

onMounted(async () => {
  await listen("load-theme", (_event) => {
    accentPrimary.value = props.theme.accent_primary;
  });
});

</script>

<template>
    <div class="mr-2">
        <div class="flex flex-col flex-grow ml-2">
            <div class="title">{{ props.title }}</div>
            <div>{{ props.description }}</div>
        </div>
        <div class="ml-2 mt-2 flex items-start">
            <TextArea :value="value" :theme="props.theme" :placeholder="placeholder"
                @on-change="emit('updateValue', $event)" />
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
