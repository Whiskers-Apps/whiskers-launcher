<script setup lang="ts">
import { onMounted, ref } from "vue";
import { ExtensionManifest, Theme } from "../../ViewModel";
import { getHexCssFilter, getIconUrl, getSettings } from "@/utils";
import InputField from "@/components/InputField.vue";
import { invoke } from "@tauri-apps/api";
import axios from "axios";
import PrimaryButton from "@/components/PrimaryButton.vue";
import { emit, listen } from "@tauri-apps/api/event";
import SecondaryButton from "@/components/SecondaryButton.vue";
import { open } from "@tauri-apps/api/shell";
import { WebviewWindow } from "@tauri-apps/api/window";

interface StoreExtension {
  id: string;
  name: string;
  description: string;
  preview: string;
  repo: string;
  os: string[];
}

const os = ref("");
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
const extensions = ref<StoreExtension[]>([]);
const filteredExtensions = ref<StoreExtension[]>([]);
const userExtensions = ref<ExtensionManifest[]>([]);
const pageExtensions = ref<StoreExtension[]>([]);
const currentPage = ref(0);
const installingExtension = ref(false);

onMounted(async () => {
  const settings = await getSettings();

  os.value = await invoke("get_os");

  appTheme.value = settings.theme;
  backgroundMain.value = appTheme.value.background_main;
  backgroundSecondary.value = appTheme.value.background_secondary;
  backgroundTertiary.value = appTheme.value.background_tertiary;
  accentPrimary.value = appTheme.value.accent_primary;
  textOnPrimary.value = appTheme.value.text_on_primary;
  textOnBackground.value = appTheme.value.text_on_background;

  hasLoaded.value = true;

  userExtensions.value = await invoke("get_user_extensions");
  extensions.value = await invoke("get_cached_extensions_store");
  filteredExtensions.value = extensions.value;
  pageExtensions.value = filteredExtensions.value.slice(0, 12);

  axios
    .get(
      "https://raw.githubusercontent.com/lighttigerXIV/whiskers-launcher-extensions/main/extensions.json"
    )
    .then((response) => {
      extensions.value = response.data;
      filteredExtensions.value = extensions.value;
      pageExtensions.value = extensions.value.slice(0, 12);

      invoke("cache_extensions", { extensions: extensions.value });
    })
    .catch((e) => {
      console.error(e);
    });
});

function canShowExtension(extensionOS: string[]): boolean {
  return extensionOS.includes("*") || extensionOS.includes(os.value);
}

function isInstalled(extensionId: string): boolean {
  return userExtensions.value.some((ue) => ue.id === extensionId);
}

async function installExtension(id: string) {
  let repo = filteredExtensions.value.find((e) => e.id === id)!!.repo;
  installingExtension.value = true;

  await invoke("clone_extension", { url: repo });

  installingExtension.value = false;

  userExtensions.value = await invoke("get_user_extensions");

  emit("load-extensions");
}

async function uninstallExtension(extensionId: string) {
  new WebviewWindow("confirm-uninstall-extension", {
    url: "confirm-uninstall-extension",
    title: "Uninstall Extension",
    width: 1000,
    height: 200,
    resizable: false,
    center: true,
  });

  const unlisten = await listen("confirm-uninstall-extension", async (_event) => {
    await invoke("uninstall_extension", {
      extension_id: extensionId,
    });

    userExtensions.value = await invoke("get_user_extensions");
    emit("load-extensions");

    unlisten();
  });
}

async function search() {
  filteredExtensions.value = extensions.value.filter((theme) =>
    theme.name.trim().toLowerCase().includes(searchText.value.trim().toLowerCase())
  );

  pageExtensions.value = filteredExtensions.value;
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
  return filteredExtensions.value.length - 1 > currentPage.value * 12 + 12;
}

function refreshPage() {
  if (currentPage.value === 0) {
    pageExtensions.value = filteredExtensions.value.slice(0, 12);
  } else {
    const start = currentPage.value * 12;
    const end = start + 12;

    pageExtensions.value = filteredExtensions.value.slice(start, end);
  }
}
</script>
<template>
  <div v-if="hasLoaded" class="main h-screen max-h-screen p-4 flex flex-col items-center">
    <div class="flex justify-center w-full max-w-[1000px]">
      <InputField
        placeholder="Search extension"
        :value="searchText"
        :theme="appTheme!!"
        @on-change="
          searchText = $event;
          if (searchText === '') {
            filteredExtensions = extensions;
            pageExtensions = filteredExtensions.slice(0, 12);
          }
        "
        @on-enter="
          searchText = $event;
          search();
        "
      />
    </div>
    <div
      class="flex-grow mt-4 w-full overflow-auto grid md:grid-cols-3 sm:grid-cols-2 gap-4 justify-center max-w-[1000px]"
      ref="themesGrid"
    >
      <div v-for="extension in pageExtensions" :key="extension.id" :id="extension.id">
        <div v-if="canShowExtension(extension.os)" class="p-4 extension-card rounded-2xl h-fit">
          <div class="background-tertiary rounded-2xl">
            <img
              :id="`preview-${extension.id}`"
              :src="extension.preview"
              class="rounded-lg aspect-square w-full object-contain"
            />
          </div>
          <div class="flex flex-col items-center justify-center p-2">
            <div class="font-semibold text-xl w-full text-start one-line-text">
              {{ extension.name }}
            </div>
            <div class="flex-grow"></div>
            <div class="flex justify-end w-full mt-2">
              <SecondaryButton
                class="w-full mr-2"
                text="Source"
                :theme="appTheme!!"
                @click="open(extension.repo)"
                :disabled="installingExtension"
              />
              <PrimaryButton
                v-if="isInstalled(extension.id)"
                class="w-full"
                text="Uninstall"
                :theme="appTheme!!"
                @click="uninstallExtension(extension.id)"
                :disabled="installingExtension"
              />

              <PrimaryButton
                v-else
                class="w-full"
                text="Install"
                :theme="appTheme!!"
                @click="installExtension(extension.id)"
                :disabled="installingExtension"
              />
            </div>
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
::-webkit-scrollbar {
  width: 4px;
}

::-webkit-scrollbar-track {
  margin-top: 16px;
  margin-bottom: 16px;
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: v-bind(accentPrimary);
  border-radius: 48px;
}
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

.extension-card {
  background-color: v-bind(backgroundSecondary);
}

.page-button {
  background-color: v-bind(accentPrimary);
}

.page-button:disabled {
  background-color: v-bind(backgroundTertiary);
}
</style>
