<script setup lang="ts">
import { appWindow, LogicalSize, WebviewWindow } from "@tauri-apps/api/window"
import { register } from "@tauri-apps/api/globalShortcut"
import { event, invoke } from "@tauri-apps/api";
import { onMounted, ref, watch } from "vue";
import SearchSVG from "./assets/icons/search.svg"
import SettingsSVG from "./assets/icons/settings.svg"
import { getSettings, getRoundnessInPixels } from "./pages/Settings/Settings";
import { listen } from "@tauri-apps/api/event"



const showSearchIcon = ref();
const showSettingsIcon = ref();
const roundnessLevel = ref();
const borderWidth = ref();
const searchText = ref("")
const backgroundColor = ref("")
const secondaryBackgroundColor = ref("")
const accentColor = ref("")
const textColor = ref("")
const secondaryTextColor = ref("")
const searchRef = ref();
const focus = ref();
const results = ref();

appWindow.onFocusChanged(action => {
  if (action.event == "tauri://blur") {
    searchText.value = "";
  }
})


function openSettings() {
  const webview = new WebviewWindow('settings', {
    url: 'settings',
    title: "Settings",
    width: 1100
  })

  webview.once('tauri://created', function () { })
  webview.once('tauri://error', function (e) {
    console.error(e);
  })
}



onMounted(async () => {

  let settings = await getSettings();

  roundnessLevel.value = getRoundnessInPixels(settings.search_box.roundness);
  borderWidth.value = `${settings.search_box.border_width}px`
  showSearchIcon.value = settings.search_box.show_search_icon;
  showSettingsIcon.value = settings.search_box.show_settings_icon;
  backgroundColor.value = settings.theming.background;
  secondaryBackgroundColor.value = settings.theming.secondary_background;
  accentColor.value = settings.theming.accent;
  textColor.value = settings.theming.text;
  secondaryTextColor.value = settings.theming.seconday_text;

  searchRef.value.focus();

  focus.value = await listen("focus_box", (event) => {
    appWindow.setFocus();
    searchRef.value.focus();
  })
})

document.addEventListener('keydown', function (event) {
  if (event.ctrlKey && event.key === 's') {
    openSettings();
  }

  if (event.ctrlKey && event.key === 'r') {
    document.location.reload()
  }

  if (event.key === "Escape") {
    appWindow.hide();
  }

  if (event.key === "Enter") {
    invoke("hide_window");
  }

  if (event.altKey && event.key == "Space") {
    invoke("show_window");
  }
});

const searchBoxHeight = ref(70);

watch(searchText, async (newText, oldText) => {

  results.value = await invoke("get_results");
  console.log(JSON.parse(results.value));
})

</script>

<template>
  <div class="items-center justify-center h-screen w-screen bg-transparent text">

    <div class=" flex flex-col pt-2 pb-2 pl-4 pr-4 overflow-clip searchBox">
      <div class="flex flex-grow">
        <button v-if="showSearchIcon">
          <SearchSVG class="w-5 h-5 mr-4 stroke" />
        </button>

        <input ref="searchRef" class="flex-grow bg-transparent border-none outline-none h-full placeholder"
          v-model="searchText" placeholder="Search Anything">

        <button v-if="showSettingsIcon" class="p-1 rounded-full secondaryHover ml-4" @click="openSettings()">
          <SettingsSVG class="h-5 w-5 stroke" />
        </button>
      </div>

      <div class="flex flex-col">
        <div v-for="char in searchText">
          {{ char }}
        </div>
      </div>

    </div>


  </div>
</template>

<style scoped>
.text {
  color: v-bind(accentColor)
}

.searchBox {
  background-color: v-bind(backgroundColor);
  border: solid v-bind(borderWidth) v-bind(accentColor);
  font-size: 1.1rem;
  border-radius: v-bind(roundnessLevel);
  height: v-bind(searchBoxHeight);
}

.placeholder::placeholder {
  color: v-bind(secondaryTextColor);
}

::selection {
  background-color: v-bind(secondaryBackgroundColor);
}

.secondaryHover:hover {
  background-color: v-bind(secondaryBackgroundColor);
}

.stroke {
  fill: none;
  stroke: v-bind(accentColor);
  stroke-width: 2;
}
</style>