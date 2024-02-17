<script setup lang="ts">
import { onMounted, ref } from "vue";
import { ViewModel, SettingsTab } from "./ViewModel";
import { getHexCssFilter, getIconUrl } from "@/utils";
import { listen } from "@tauri-apps/api/event";

let props = defineProps<{
  vm: ViewModel;
}>();

let backgroundTertiary = ref("");
let accentPrimary = ref("");
let textOnBackground = ref("");
let textOnPrimary = ref("");
let accentPrimaryFilter = ref("");
let onPrimaryFilter = ref("");
let hoveringMenuButton = ref(false);

onMounted(async () => {
  loadTheme();
  await listen("load-theme", (_event) => {
    loadTheme();
  });
});

function loadTheme() {
  backgroundTertiary.value = props.vm.settings!!.theme.background_tertiary;
  accentPrimary.value = props.vm.settings!!.theme.accent_primary;
  textOnBackground.value = props.vm.settings!!.theme.text_on_background;
  textOnPrimary.value = props.vm.settings!!.theme.text_on_primary;
  accentPrimaryFilter.value = getHexCssFilter(accentPrimary.value);
  onPrimaryFilter.value = getHexCssFilter(textOnPrimary.value);
}
</script>

<template>
  <div class="flex flex-col">
    <div class="p-2" v-if="!vm.showNavbar">
      <button
        class="menu-button"
        :class="hoveringMenuButton ? 'menu-button-hover' : ''"
        @click="vm.showNavbar = !vm.showNavbar"
        @mouseover="hoveringMenuButton = true"
        @mouseout="hoveringMenuButton = false"
      >
        <img
          class="menu-button-icon"
          :class="hoveringMenuButton ? 'menu-button-icon-hover' : ''"
          width="40"
          :src="getIconUrl('menu.svg')"
          :style="{ filter: hoveringMenuButton ? onPrimaryFilter : accentPrimaryFilter }"
        />
      </button>
    </div>

    <div
      v-if="vm.showNavbar"
      class="p-2 background-secondary rounded-2xl w-[300px] flex-grow overflow-auto"
    >
      <button
        class="menu-button"
        :class="hoveringMenuButton ? 'menu-button-hover' : ''"
        @click="vm.showNavbar = !vm.showNavbar"
        @mouseover="hoveringMenuButton = true"
        @mouseout="hoveringMenuButton = false"
      >
        <img
          class="menu-button-icon"
          :class="hoveringMenuButton ? 'menu-button-icon-hover' : ''"
          width="40"
          :src="getIconUrl('menu-close.svg')"
          :style="{ filter: hoveringMenuButton ? onPrimaryFilter : accentPrimaryFilter }"
        />
      </button>

      <div class="mt-1"></div>

      <div
        class="tab"
        @click="vm.openTab(SettingsTab.General)"
        :class="vm.getTabClasses(SettingsTab.General)"
      >
        General
      </div>

      <div class="mt-1"></div>

      <div
        class="tab"
        @click="vm.openTab(SettingsTab.SearchBox)"
        :class="vm.getTabClasses(SettingsTab.SearchBox)"
      >
        Search Box
      </div>

      <div class="mt-1"></div>

      <div
        class="tab"
        @click="vm.openTab(SettingsTab.SearchResults)"
        :class="vm.getTabClasses(SettingsTab.SearchResults)"
      >
        Search Results
      </div>

      <div class="mt-1"></div>

      <div
        class="tab"
        @click="vm.openTab(SettingsTab.SearchEngines)"
        :class="vm.getTabClasses(SettingsTab.SearchEngines)"
      >
        Search Engines
      </div>

      <div class="mt-1"></div>

      <div
        class="tab"
        @click="vm.openTab(SettingsTab.Theming)"
        :class="vm.getTabClasses(SettingsTab.Theming)"
      >
        Theming
      </div>

      <div class="mt-1"></div>

      <div
        class="tab"
        @click="vm.openTab(SettingsTab.Extensions)"
        :class="vm.getTabClasses(SettingsTab.Extensions)"
      >
        Extensions
      </div>

      <div class="mt-1"></div>

      <div
        class="tab"
        @click="vm.openTab(SettingsTab.About)"
        :class="vm.getTabClasses(SettingsTab.About)"
      >
        About
      </div>
    </div>
  </div>
</template>

<style scoped>
.tab {
  font-size: 20px;
  font-weight: 600;
  padding: 8px 12px 8px 12px;
  border-radius: 48px;
  cursor: pointer;
}

.tab:hover {
  margin-left: 8px;
  color: v-bind(textOnBackground);
  background-color: v-bind(backgroundTertiary);
}

.menu-button {
  border-radius: 48px;
  padding: 8px;
}

.menu-button-hover {
  background-color: v-bind(accentPrimary);
}
</style>
