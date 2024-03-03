<script setup lang="ts">
import { PropType, onMounted, ref } from "vue";
import { SelectOption } from "@components/ComponentClasses";
import SelectField from "./SelectField.vue";
import { Theme } from "@/pages/Settings/ViewModel";
import { listen } from "@tauri-apps/api/event";

const emit = defineEmits(["updateValue"]);

const props = defineProps<{
    title: string,
    description: string,
    options: SelectOption[],
    value: string,
    theme: Theme,
    useSecondaryBackground?: boolean;
    id?: string
}>();

const accentPrimary = ref(props.theme.accent_primary);

onMounted(async () => {
  await listen("load-theme", (_event) => {
    accentPrimary.value = props.theme.accent_primary;
  });
});

</script>

<template>
    <div class="mr-2 ml-2">
        <div>
            <div class="title">{{ title }}</div>
            <div>{{ description }}</div>
        </div>
        <SelectField class="mt-2" :use-secondary-background="useSecondaryBackground" :value="value" :options="options" :theme="theme" :id="id ? id : ''"
            @update-value="emit('updateValue', $event)" />
    </div>
</template>

<style scoped>
.title {
    color: v-bind(accentPrimary);
    font-size: 24px;
    font-weight: 700;
}
</style>
