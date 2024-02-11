<script setup lang="ts">
import { getHexCssFilter, getIconUrl } from "@/utils";
import { Theme } from "@pages/Settings/ViewModel";
import { ref } from "vue";

const emit = defineEmits(["onClick"]);

const props = defineProps<{
  title: string;
  description: string;
  value: string;
  placeholder?: string;
  selectDir: boolean;
  theme: Theme;
}>();

const accentPrimary = ref(props.theme.accent_primary);
const accentPrimaryFilter = ref(getHexCssFilter(props.theme.accent_primary));
const backgroundSecondary = ref(props.theme.background_secondary);
</script>

<template>
  <div class="mr-2">
    <div class="flex flex-col flex-grow ml-2">
      <div class="title">{{ props.title }}</div>
      <div>{{ props.description }}</div>
    </div>
    <button
      class="ml-2 mt-2 flex items-start button w-full p-3 rounded-2xl hover:opacity-95"
      @click="emit('onClick')"
    >
      <img class="icon" :src="getIconUrl('directory.svg')" />
      <div class="ml-2 one-line-text">
        {{
          value === "" ? `Select ${selectDir ? "directory" : "file"}` : value
        }}
      </div>
    </button>
  </div>
</template>

<style scoped>
.title {
  color: v-bind(accentPrimary);
  font-size: 24px;
  font-weight: 700;
}

.icon {
  width: 32px;
  filter: v-bind(accentPrimaryFilter);
}

.button {
  background-color: v-bind(backgroundSecondary);
}
</style>
