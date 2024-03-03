<script setup lang="ts">
import { Theme } from "@pages/Settings/ViewModel";
import { listen } from "@tauri-apps/api/event";
import { PropType, onMounted, ref } from "vue";

const emit = defineEmits(["click"]);

const props = defineProps<{
  text: string;
  theme: Theme;
  disabled?: boolean;
  fill?: boolean
}>();

const backgroundTertiary = ref(props.theme.background_tertiary);
const accentPrimary = ref(props.theme.accent_primary);

onMounted( async ()=>{
  await listen("load-theme", (_event)=>{
    backgroundTertiary.value = props.theme.background_tertiary;
    accentPrimary.value = props.theme.accent_primary;
  });
});
</script>

<template>
  <button
    class="secondary-button rounded-full h-fit w-fit pt-2 pb-2 pl-3 pr-3"
    :class="fill ? 'w-full' : ''"
    :disabled="disabled"
    @click="emit('click')"
  >
    {{ text }}
  </button>
</template>

<style scoped>
.secondary-button {
  background-color: transparent;
  color: v-bind(accentPrimary);
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
