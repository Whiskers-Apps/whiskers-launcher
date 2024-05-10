<script setup lang="ts">
import { Theme } from "@/pages/Settings/ViewModel";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";

const emit = defineEmits(["onChange", "onEnter"]);

const props = defineProps<{
  value: string;
  placeholder?: string | null;
  theme: Theme;
  useBackgroundTertiary?: boolean;
}>();

const background = ref("");
const accentPrimary = ref("");
const textOnBackground = ref("");

onMounted(async () => {
  loadTheme();

  listen("load-theme", (_event) => {
    loadTheme();
  });
});

function loadTheme() {
  background.value = props.useBackgroundTertiary
    ? props.theme.background_tertiary
    : props.theme.background_secondary;
  accentPrimary.value = props.theme.accent_primary;
  textOnBackground.value = props.theme.text_on_background;
}
</script>
<template>
  <input
    class="input w-full p-2 pl-3 pr-3 rounded-full"
    :value="value"
    :placeholder="placeholder ?? ''"
    @input="emit('onChange', ($event.target as HTMLInputElement).value)"
    @keyup.enter="emit('onEnter', ($event.target as HTMLInputElement).value)"
  />
</template>

<style scoped>
.input {
  background-color: v-bind(background);
}

.input::placeholder {
  color: v-bind(textOnBackground);
}

.input:focus {
  outline: 1px solid v-bind(accentPrimary);
}
</style>
