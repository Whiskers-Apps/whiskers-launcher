<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Theme } from "../../ViewModel";
import { getHexCssFilter, getIconUrl, getSettings } from "@/utils";
import InputField from "@/components/InputField.vue";
import { invoke } from "@tauri-apps/api";
import axios from "axios";
import SelectField from "@/components/SelectField.vue";
import { SelectOption } from "@/components/ComponentClasses";
import PrimaryButton from "@/components/PrimaryButton.vue";
import { emit } from "@tauri-apps/api/event";

interface StoreTheme {
  name: string;
  repo: string;
  preview: string;
  variants: StoreThemeVariant[];
  file: string | null;
}

interface StoreThemeVariant {
  name: string;
  file: string;
}

interface StoreThemeSelectValue {
  themeIndex: number;
  value: string;
}

const hasLoaded = ref(false);
const appTheme = ref<Theme>();
const backgroundMain = ref("");
const backgroundSecondary = ref("");
const backgroundTertiary = ref("");
const textOnBackground = ref("");
const textOnPrimary = ref("");
const accentPrimary = ref("");

const themesGrid = ref();

const searchText = ref("");
const themes = ref<StoreTheme[]>([]);
const filteredThemes = ref<StoreTheme[]>([]);
const pageThemes = ref<StoreTheme[]>([]);
const selectValues = ref<StoreThemeSelectValue[]>([]);
const currentPage = ref(0);
const applyingTheme = ref(false);

onMounted(async () => {
  const settings = await getSettings();
  appTheme.value = settings.theme;
  backgroundMain.value = appTheme.value.background_main;
  backgroundSecondary.value = appTheme.value.background_secondary;
  backgroundTertiary.value = appTheme.value.background_tertiary;
  accentPrimary.value = appTheme.value.accent_primary;
  textOnPrimary.value = appTheme.value.text_on_primary;
  textOnBackground.value = appTheme.value.text_on_background;

  hasLoaded.value = true;

  themes.value = await invoke("get_cached_themes_store");
  filteredThemes.value = themes.value;
  pageThemes.value = filteredThemes.value.slice(0, 12);

  await initSelectValues();
  /*
  if (await invoke("has_internet")) {
    axios
      .get(
        "https://raw.githubusercontent.com/lighttigerXIV/whiskers-launcher-themes/master/themes.json"
      )
      .then(async (response) => {
        themes.value = response.data;
        currentThemes.value = themes.value;
        
        await initSelectValues();
        invoke("cache_themes", { themes: themes.value });
      })
      .catch((e) => {
        console.error(e);
      });
  }
  */
});

async function initSelectValues() {
  selectValues.value = [];

  filteredThemes.value.forEach((theme, index) => {
    selectValues.value.push({
      themeIndex: index,
      value: theme.file ? theme.file : theme.variants[0].file,
    });
  });
}

function getThemeVariants(variants: StoreThemeVariant[]): SelectOption[] {
  let options: SelectOption[] = [];

  variants.forEach((variant) => {
    options.push({
      id: variant.file,
      text: variant.name,
    });
  });

  return options;
}

function getStoreThemeSelectValue(theme: StoreTheme, index: number): string {
  selectValues.value.forEach((selectValue) => {
    if (selectValue.themeIndex === index) {
      return selectValue.value;
    }
  });

  return theme.variants[0].file;
}

function updateStoreThemeSelectValue(theme: StoreTheme, index: number, value: string) {
  let newValues: StoreThemeSelectValue[] = [];
  let found = false;

  selectValues.value.forEach((selectValue) => {
    if (selectValue.themeIndex === index) {
      found = true;

      newValues.push({
        themeIndex: index,
        value: value,
      });
    } else {
      newValues.push(selectValue);
    }
  });

  if (!found) {
    newValues.push({
      themeIndex: index,
      value: theme.variants[0].file,
    });
  }

  selectValues.value = newValues;
}

async function applyTheme(index: number) {
  let file = selectValues.value[index].value;

  applyingTheme.value = true;

  await invoke("apply_store_theme", { file: file });

  applyingTheme.value = false;

  emit("load-settings");

  const settings = await getSettings();

  appTheme.value = settings.theme;
  backgroundMain.value = appTheme.value.background_main;
  backgroundSecondary.value = appTheme.value.background_secondary;
  backgroundTertiary.value = appTheme.value.background_tertiary;
  textOnBackground.value = appTheme.value.text_on_background;
  accentPrimary.value = appTheme.value.accent_primary;
  textOnPrimary.value = appTheme.value.text_on_primary;
}

async function search() {
  filteredThemes.value = themes.value.filter((theme) =>
    theme.name.trim().toLowerCase().includes(searchText.value.trim().toLowerCase())
  );

  pageThemes.value = filteredThemes.value;
}

function getPreviousPageButtonFilter(): string {
  return currentPage.value === 0
    ? getHexCssFilter(textOnBackground.value)
    : getHexCssFilter(textOnPrimary.value);
}

function getNextPageButtonFilter(): string {
  return hasNextPage()
    ? getHexCssFilter(textOnPrimary.value)
    : getHexCssFilter(textOnBackground.value);
}

function hasNextPage(): boolean {
  return filteredThemes.value.length - 1 > currentPage.value * 12 + 12;
}

function refreshPage() {
  if (currentPage.value === 0) {
    pageThemes.value = filteredThemes.value.slice(0, 12);
  } else {
    const start = currentPage.value * 12;
    const end = start + 12;

    pageThemes.value = filteredThemes.value.slice(start, end);
  }
}
</script>
<template>
  <div v-if="hasLoaded" class="main h-screen max-h-screen p-4 flex flex-col items-center">
    <div class="flex justify-center w-full max-w-[1000px]">
      <InputField
        placeholder="Search theme"
        :value="searchText"
        :theme="appTheme!!"
        @on-change="
          searchText = $event;
          if (searchText === '') {
            filteredThemes = themes;
            pageThemes = filteredThemes.slice(0, 12);
          }
        "
        @on-enter="
          searchText = $event;
          search();
        "
      />
    </div>
    <div
      class="flex-grow mt-4 overflow-auto grid md:grid-cols-3 sm:grid-cols-2 gap-4 justify-center max-w-[1000px]"
      ref="themesGrid"
    >
      <div
        v-for="(theme, index) in pageThemes"
        class="p-4 theme-card rounded-2xl h-fit"
        :key="theme.repo"
        :id="theme.repo"
      >
        <div class="background-tertiary rounded-2xl w-full aspect-square">
          <img
            :id="`preview-${theme.name}-${theme.preview}`"
            :src="theme.preview"
            class="rounded-lg aspect-square object-contain"
          />
        </div>
        <div class="flex flex-col items-center justify-center p-2">
          <div class="font-semibold text-xl w-full text-start one-line-text">{{ theme.name }}</div>
          <SelectField
            class="mt-2"
            v-if="theme.variants.length > 0"
            :value="getStoreThemeSelectValue(theme, index)"
            :options="getThemeVariants(theme.variants)"
            :theme="appTheme!!"
            @update-value="updateStoreThemeSelectValue(theme, index, $event)"
          />
          <div class="flex-grow"></div>
          <div class="flex justify-end w-full mt-2">
            <PrimaryButton
              class="w-full"
              text="Apply"
              :theme="appTheme!!"
              @click="applyTheme(index)"
              :disabled="applyingTheme"
            />
          </div>
        </div>
      </div>
    </div>
    <div class="w-full flex justify-end mt-4 max-w-[1000px]">
      <div class="p-2 background-secondary rounded-full flex">
        <button
          class="page-button rounded-full p-3 w-[50px] h-[50px] flex items-center justify-center"
          :disabled="currentPage <= 0"
          @click="
            currentPage -= 1;
            refreshPage();
          "
        >
          <img
            :src="getIconUrl('chevron-left.svg')"
            width="24"
            :style="{ filter: getPreviousPageButtonFilter() }"
          />
        </button>
        <div
          class="p-3 background-tertiary ml-2 mr-2 rounded-full w-[50px] h-[50px] flex items-center justify-center"
        >
          {{ currentPage + 1 }}
        </div>
        <button
          class="page-button rounded-full w-[50px] h-[50px] flex items-center justify-center"
          :disabled="!hasNextPage()"
          @click="
            currentPage += 1;
            refreshPage();
          "
        >
          <img
            :src="getIconUrl('chevron-right.svg')"
            width="24"
            :style="{ filter: getNextPageButtonFilter() }"
          />
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.main {
  background-color: v-bind(backgroundMain);
  color: v-bind(textOnBackground);
}

.background-secondary {
  background-color: v-bind(backgroundSecondary);
}

.background-tertiary {
  background-color: v-bind(backgroundTertiary);
}

.theme-card {
  background-color: v-bind(backgroundSecondary);
}

.page-button {
  background-color: v-bind(accentPrimary);
}

.page-button:disabled {
  background-color: v-bind(backgroundTertiary);
}
</style>
