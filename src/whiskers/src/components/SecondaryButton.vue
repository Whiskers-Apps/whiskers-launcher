<script setup lang="ts">
import { Theme } from "@pages/Settings/ViewModel";
import { PropType, ref } from "vue";

const emit = defineEmits(["click"]);

const props = defineProps<{
  text: string;
  theme: Theme;
  disabled?: boolean;
}>();

const backgroundTertiary = ref(props.theme.background_tertiary);
const accentPrimary = ref(props.theme.accent_primary);
const textOnBackground = ref(props.theme.text_on_background);
</script>

<template>
  <button
    class="secondary-button rounded-full h-fit w-fit pt-2 pb-2 pl-3 pr-3"
    :disabled="disabled"
    @click="emit('click')"
  >
    {{ text }}
  </button>
</template>

<style scoped>
.secondary-button {
  background-color: v-bind(backgroundTertiary);
  color: v-bind(textOnBackground);
  border: 1px solid v-bind(accentPrimary);
}

.secondary-button:hover:enabled {
  filter: brightness(0.95);
  cursor: pointer;
}

.secondary-button:disabled {
  background-color: v-bind(backgroundTertiary);
}
</style>
