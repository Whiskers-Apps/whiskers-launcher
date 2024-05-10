<script setup lang="ts">
import { onMounted, ref } from "vue";
import { SettingsTab, ViewModel } from "./ViewModel";
import Navbar from "./NavBar.vue";
import GeneralTab from "./GeneralTab.vue";
import SearchResultsTab from "./SearchResultsTab.vue";
import SearchBoxTab from "./SearchBoxTab.vue";
import SearchEnginesTab from "./SearchEnginesTab.vue";
import ThemingTab from "./ThemingTab.vue";
import ExtensionsTab from "./ExtensionsTab.vue";
import AboutTab from "./AboutTab.vue";
import { listen } from "@tauri-apps/api/event";

let vm = ref<ViewModel>(new ViewModel());

const backgroundMain = ref("");
const backgroundSecondary = ref("");
const backgroundTertiary = ref("");
const accentPrimary = ref("");
const accentDanger = ref("");
const textOnBackground = ref("");
const textOnPrimary = ref("");
const textOnDanger = ref("");

onMounted(async () => {
  await vm.value.load();
  loadTheme();

  vm.value.hasLoaded = true;

  await listen("load-theme", (_event) => {
    loadTheme();
  });
});

function loadTheme() {
  let theme = vm.value.settings!!.theme;

  backgroundMain.value = theme.background_main;
  backgroundSecondary.value = theme.background_secondary;
  backgroundTertiary.value = theme.background_tertiary;
  accentPrimary.value = theme.accent_primary;
  accentDanger.value = theme.accent_danger;
  textOnBackground.value = theme.text_on_background;
  textOnPrimary.value = theme.text_on_primary;
  textOnDanger.value = theme.text_on_danger;
}
</script>

<template>
  <div
    v-if="vm.hasLoaded"
    class="p-4 text-on-background background-main flex h-screen max-h-screen overflow-clip font-medium"
  >
    <Navbar :vm="(vm as ViewModel)" />
    <div class="flex-grow max-w-[900px] flex flex-col">
      <div class="flex-grow overflow-auto p-2 pl-4 pr-4">
        <GeneralTab v-if="vm.selectedTab == SettingsTab.General" :vm="(vm as ViewModel)" />
        <SearchBoxTab v-if="vm.selectedTab == SettingsTab.SearchBox" :vm="(vm as ViewModel)" />
        <SearchResultsTab
          v-if="vm.selectedTab == SettingsTab.SearchResults"
          :vm="(vm as ViewModel)"
        />
        <SearchEnginesTab
          v-if="vm.selectedTab == SettingsTab.SearchEngines"
          :vm="(vm as ViewModel)"
        />
        <ThemingTab v-if="vm.selectedTab == SettingsTab.Theming" :vm="(vm as ViewModel)" />
        <ExtensionsTab v-if="vm.selectedTab == SettingsTab.Extensions" :vm="(vm as ViewModel)" />
        <AboutTab v-if="vm.selectedTab == SettingsTab.About" :vm="(vm as ViewModel)" />
      </div>
    </div>
  </div>
</template>

<style>
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

.background-main {
  background-color: v-bind(backgroundMain);
}

.background-secondary {
  background-color: v-bind(backgroundSecondary);
}

.background-primary {
  background-color: v-bind(accentPrimary);
}

.background-danger {
  background-color: v-bind(accentDanger);
}

.text-primary {
  color: v-bind(accentPrimary);
}

.text-on-background {
  color: v-bind(textOnBackground);
}

.text-on-primary {
  color: v-bind(textOnPrimary);
}

.text-on-danger {
  color: v-bind(textOnDanger);
}

.title {
  color: v-bind(accentPrimary);
  font-size: 24px;
  font-weight: 700;
}

.divider {
  height: 2px;
  width: 100%;
  background-color: v-bind(backgroundTertiary);
}
</style>
