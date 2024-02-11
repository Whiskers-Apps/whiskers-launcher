<script setup lang="ts">
import { Theme } from "@pages/Settings/ViewModel";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";

const emit = defineEmits(["click"]);

const props = defineProps<{
  theme: Theme;
  text: string;
  disabled?: boolean;
  danger?: boolean;
}>();

const backgroundTertiary = ref(props.theme.background_tertiary);
const accent = ref(
  props.danger ? props.theme.accent_danger : props.theme.accent_primary,
);
const textColor = ref(
  props.danger ? props.theme.text_on_danger : props.theme.text_on_primary,
);

const textOnBackground = ref(props.theme.text_on_background);


onMounted(async () =>{
    await listen("refresh-theme", (_event) => {
        backgroundTertiary.value = props.theme.background_tertiary;
        textOnBackground.value = props.theme.text_on_background;
        accent.value = props.danger ? props.theme.accent_danger : props.theme.accent_primary;
        textColor.value = props.danger ? props.theme.text_on_danger : props.theme.text_on_primary

    });
});

</script>

<template>
  <button
    class="primary-button rounded-full h-fit w-fit pt-2 pb-2 pl-3 pr-3"
    :disabled="disabled"
    @click="emit('click')"
  >
    {{ text }}
  </button>
</template>

<style scoped>
i .primary-button,
.primary-button:enabled {
  background-color: v-bind(accent);
  color: v-bind(textColor);
}

.primary-button:hover:enabled {
  filter: brightness(0.95);
  cursor: pointer;
}

.primary-button:disabled {
  background-color: v-bind(backgroundTertiary);
  color: v-bind(textOnBackground);
}

.primary-button:focus{
  outline: 1px solid v-bind(accent);
}
</style>
