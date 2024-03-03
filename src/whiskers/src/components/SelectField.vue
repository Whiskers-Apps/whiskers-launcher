<script setup lang="ts">
import { Theme } from "@pages/Settings/ViewModel";
import { getHexCssFilter, getIconUrl } from "@/utils";
import { onMounted, ref } from "vue";
import { SelectOption } from "./ComponentClasses";
import { listen } from "@tauri-apps/api/event";

const props = defineProps<{
  value: string;
  options: SelectOption[];
  theme: Theme;
  useSecondaryBackground?: boolean;
  id?: string;
}>();

const emit = defineEmits(["updateValue"]);

const input = ref(props.options.find((option) => option.id === props.value)?.text ?? "");
const showOptions = ref(false);
const selectedIndex = ref(props.options.findIndex((it) => it.id === props.value) ?? 0);
const currentOptions = ref<SelectOption[]>(props.options);

const randomSelectId = Math.random() * 1000000;

const backgroundSecondary = ref(props.theme.background_secondary);
const backgroundTertiary = ref(props.theme.background_tertiary);
const textOnBackground = ref(props.theme.text_on_background);
const accentPrimary = ref(props.useSecondaryBackground ? backgroundSecondary.value : backgroundTertiary.value);
const background = ref(props.useSecondaryBackground ? backgroundSecondary.value : backgroundTertiary.value);
const highlightBackground = ref(props.useSecondaryBackground ? backgroundTertiary.value : backgroundSecondary.value);

onMounted(async () => {
  loadTheme();

  listen("load-theme", (_event) => {
    loadTheme();
  });
});

function loadTheme() {
  backgroundSecondary.value = props.theme.background_secondary;
  backgroundTertiary.value = props.theme.background_tertiary;
  textOnBackground.value = props.theme.text_on_background;
  accentPrimary.value = props.theme.accent_primary;
  background.value = props.useSecondaryBackground ? backgroundSecondary.value : backgroundTertiary.value;
  highlightBackground.value = props.useSecondaryBackground ? backgroundTertiary.value : backgroundSecondary.value;
}

document.addEventListener("keydown", function (event) {
  if (event.key === "ArrowDown") {
    event.preventDefault();

    if (selectedIndex.value < currentOptions.value.length - 1) {
      selectedIndex.value += 1;

      const divElement = this.getElementById(randomSelectId.toString())?.getElementsByClassName(
        "option"
      )[selectedIndex.value];
      divElement?.scrollIntoView();
    }
  }

  if (event.key === "ArrowUp") {
    event.preventDefault();

    if (selectedIndex.value > 0) {
      selectedIndex.value -= 1;

      const divElement = this.getElementById(randomSelectId.toString())?.getElementsByClassName(
        "option"
      )[selectedIndex.value];
      divElement?.scrollIntoView();
    }
  }
});

function updateValue(option: SelectOption) {
  if (option.id !== "") {
    emit("updateValue", option.id);
    showOptions.value = false;

    selectedIndex.value = currentOptions.value.findIndex((it) => it.id === option.id) ?? 0;

    currentOptions.value = props.options;
    input.value = option.text;
  }
}

function filter() {
  showOptions.value = true;

  let newOptions = props.options.filter((it) =>
    it.text.toLowerCase().includes(input.value.toLowerCase())
  );

  if (newOptions.length === 0) {
    newOptions.push({
      id: "",
      text: "Couldn't find any value",
    });
  }

  currentOptions.value = newOptions;

  selectedIndex.value = 0;
}
</script>

<template>
  <div
    class="w-full flex flex-col"
    :id="randomSelectId.toString()"
    @keydown.enter="updateValue(currentOptions[selectedIndex])"
  >
    <div class="w-full flex select-box" @click="showOptions = !showOptions">
      <input v-model="input" class="flex-grow dropdown relative" v-on:input="filter()" />

      <div class="flex items-center justify-center chevron ml-2">
        <img
          :src="getIconUrl('chevron-down.svg')"
          :style="{ filter: getHexCssFilter(accentPrimary) }"
          width="24"
        />
      </div>
    </div>

    <div class="flex">
      <div v-if="showOptions" class="overflow-clip options-div w-full">
        <div
          v-for="(option, index) in currentOptions"
          :class="index === selectedIndex ? 'selected-option' : ''"
          :id="option.id"
          :key="option.id"
        >
          <button @click="updateValue(option)" class="w-full text-start option">
            {{ option.text }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.select-box {
  background-color: v-bind(background);
  border: 1px solid v-bind(background);
  border-radius: 48px;
  overflow: auto;
  padding: 8px 16px 8px 16px;
  cursor: pointer;
}

.select-box:focus-within {
  outline: 1px solid v-bind(accentPrimary);
}

.dropdown {
  all: unset;
  display: flex;
  width: 100%;
  cursor: pointer;
}

.options-div {
  margin-top: 16px;
  background-color: v-bind(background);
  border-radius: 8px;
  border: 1px solid v-bind(accentPrimary);
  overflow: auto;
  max-height: 400px;
}

.option {
  text-align: start;
  flex-grow: 1;
  padding: 16px;
}

.option:hover {
  background-color: v-bind(highlightBackground);
}

.selected-option {
  background-color: v-bind(highlightBackground);
}

.fill-text {
  fill: v-bind(textOnBackground);
}
</style>
