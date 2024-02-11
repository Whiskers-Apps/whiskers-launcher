<script setup lang="ts">
import { Theme } from "@/pages/Settings/ViewModel";
import { onMounted, ref } from "vue";

const emit = defineEmits(["onChange"]);

const props = defineProps<{
    value: string;
    placeholder?: string | null;
    theme: Theme;
    useBackgroundTertiary?: boolean
}>();

const background = ref( props.useBackgroundTertiary ? props.theme.background_tertiary: props.theme.background_secondary);
const accentPrimary = ref(props.theme.accent_primary);
const textOnBackground = ref(props.theme.text_on_background);

</script>
<template>
    <input class="input w-full p-2 pl-3 pr-3 rounded-full " :value="value" :placeholder="placeholder ?? ''"
        @input="emit('onChange', ($event.target as HTMLInputElement).value)" />
</template>

<style scoped>
.input{
    background-color: v-bind(background);
}

.input::placeholder{
    color: v-bind(textOnBackground);
}

.input:focus{
    outline: 1px solid v-bind(accentPrimary);
}
</style>
