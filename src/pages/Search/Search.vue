<script setup lang="ts">
import { appWindow, WebviewWindow } from "@tauri-apps/api/window"
import { invoke } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { onMounted, ref, watch } from "vue";
import SearchSVG from "../../assets/icons/search.svg";
import SettingsSVG from "../../assets/icons/settings.svg";
import { getSettings, getTheme } from "@pages/Settings/Settings";
import { SimpleKlResult } from "@/data"
import { hexToCSSFilter } from "hex-to-css-filter"
import { isIconWithTitleAndDescriptionResult, isTitleAndDescriptionResult, isIconWithTextResult, isTextResult } from "@pages/Search/SearchVM"

const showSearchIcon = ref();
const showSettingsIcon = ref();
const showPlaceholder = ref();
const roundnessLevel = ref();
const borderWidth = ref();
const searchText = ref("");
const layout = ref("");
const splitUI = ref();

const backgroundColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");
const textColor = ref("");
const secondaryTextColor = ref("");

const searchRef = ref();
const results = ref<SimpleKlResult[]>([]);
const resultsCount = ref(0)
const resultsBoxHeight = ref("");
const selectedIndex = ref(0);
const resultsRef = ref([]);


onMounted(async () => {

  searchRef.value.focus();

  let settings = await getSettings();

  roundnessLevel.value = `${settings.search.border_radius}px`;
  borderWidth.value = `${settings.search.border_width}px`
  showSearchIcon.value = settings.search.show_search_icon;
  showSettingsIcon.value = settings.search.show_settings_icon;
  showPlaceholder.value = settings.search.show_placeholder;
  resultsCount.value = settings.results.results_count;
  layout.value = settings.results.layout.type;
  splitUI.value = settings.results.split_ui;

  let theme = await getTheme();

  backgroundColor.value = theme.background;
  secondaryBackgroundColor.value = theme.secondary_background;
  accentColor.value = theme.accent;
  textColor.value = theme.text;
  secondaryTextColor.value = theme.secondary_text;

})

function openSettings() {

  appWindow.hide();

  new WebviewWindow("settings", {
    url: "settings",
    title: "Settings"
  });

}

function getCSSFilterFromHexColor(hexColor?: string): string {

  if (hexColor !== null) {

    let color = hexColor!!;

    if (hexColor === "accent") {
      color = accentColor.value
    }

    let filter = hexToCSSFilter(color);

    return filter.filter.replace(";", "")
  }

  return "none"
}

document.addEventListener('keydown', function (event) {

  if (event.key === "ArrowDown") {

    event.preventDefault(); //Prevents cursor from changing position

    if (selectedIndex.value < results.value.length - 1) {
      selectedIndex.value = selectedIndex.value + 1;
      (resultsRef.value[selectedIndex.value - 1] as HTMLDivElement).scrollIntoView({ behavior: 'smooth' });
    } else if (selectedIndex.value == results.value.length - 1) {
      selectedIndex.value = 0;
      (resultsRef.value[0] as HTMLDivElement).scrollIntoView({ behavior: 'smooth' });
    }
  }

  if (event.key === "ArrowUp") {

    event.preventDefault(); //Prevents cursor from changing position

    if (selectedIndex.value > 0) {
      selectedIndex.value = selectedIndex.value - 1;
      (resultsRef.value[selectedIndex.value - 1] as HTMLDivElement).scrollIntoView({ behavior: 'smooth' });
    } else if (selectedIndex.value == 0) {
      selectedIndex.value = results.value.length - 1;
      (resultsRef.value[selectedIndex.value - 1] as HTMLDivElement).scrollIntoView({ behavior: 'smooth' });
    }
  }

  if (event.ctrlKey && event.key === 's') {
    openSettings();
  }

  if (event.ctrlKey && event.key === 'r') {
    document.location.reload()
  }

  if (event.key === "Escape") {
    appWindow.close();
  }

  if (event.key === "Enter") {
    runAction();
  }
});

async function runAction() {

  if (results.value.length > 0) {

    var result = results.value[selectedIndex.value];
    var type = result.action?.type;

    if (type != null) {
      invoke("run_action", {
        action_type: type,
        action_json: JSON.stringify(result.action)
      });
    }
  }
}

watch(searchText, async (_newText, _oldText) => {

  if (searchText.value.trim() === "") {

    results.value = [];
    resultsBoxHeight.value = `0px`
  } else {

    results.value = await invoke("get_results", { search_text: searchText.value });

    let newHeight = 0;

    //If the results counts is bigger then the setting limit
    if (results.value.length > resultsCount.value) {
      newHeight = newHeight + (resultsCount.value * getResultHeight());
    } else {
      newHeight = newHeight + results.value.length * getResultHeight();
    }

    newHeight += 16;

    resultsBoxHeight.value = `${newHeight}px`;
    
  }

  selectedIndex.value = 0;
})

function getResultHeight(): number {
  
  switch (layout.value) {
    case "Small": { return 50 }
    case "Medium": { return 60 }
    default: { return 70 }
  }
}

function getResultHeightClass(): string {
  switch (layout.value) {
    case "Small": { return "smallResult" }
    case "Medium": { return "mediumResult" }
    default: { return "largeResult" }
  }
}

function getIconHeightClass(): string {
  switch (layout.value) {
    case "Small": { return "smallIcon" }
    case "Medium": { return "mediumIcon" }
    default: { return "largeIcon" }
  }
}

</script>

<template>
  <div class="items-center flex flex-col h-screen w-screen max-h-screen text maxHeight">
    <div :class="splitUI ? '' : 'mainBox'">
      <div class="flex items-center " :class="`${getResultHeightClass()} ${splitUI ? 'splitSearchBox' : 'searchBox'}`">
        <div v-if="showSearchIcon" class="mr-2">
          <SearchSVG class="w-5 h-5 stroke" />
        </div>
        <div class="flex-grow">
          <input ref="searchRef" class="w-full background outline-none placeholder"
            :placeholder="showPlaceholder ? 'Search' : ''" v-model="searchText" />
        </div>
        <button v-if="showSettingsIcon" class="ml-2 secondaryHover rounded-full" @click="openSettings">
          <SettingsSVG class="w-5 h-5 stroke" />
        </button>
      </div>

      <div v-if="splitUI" class="h-[10px]"></div>

      <div v-if="results.length > 0"  :class="splitUI ? 'splitResultsBox' : 'resultsBox'" :style="`height: ${resultsBoxHeight}`">
        <div v-for="(result, index) in results" ref="resultsRef">
          <div :ref="`result-${index}`" class="pl-4 pr-4 pt-2 pb-2 min-w-0 flex overflow-hidden"
            :class="`${index === selectedIndex ? 'selectedResult' : ''} ${getResultHeightClass()}`">

            <div v-if="isTextResult(result)" class="flex items-center">
              <div class="min-w-0 oneLineText">{{ result.text }}</div>
            </div>

            <div v-if="isIconWithTextResult(result)" class="flex items-center">
              <img :src="convertFileSrc(result.icon!!)" class=" object-contain h-full aspect-square icon"
                :class="getIconHeightClass()" :style="{ filter: getCSSFilterFromHexColor(result.icon_color) }">
              <div class="text-lg ml-2 flex-grow min-w-0 oneLineText">{{ result.text }}</div>
            </div>

            <div v-if="isTitleAndDescriptionResult(result)" class="flex flex-col justify-center p-2">
              <div class="font-bold min-w-0 oneLineText">{{ result.title }}
              </div>
              <div class="subtext text-xs min-w-0 oneLineText">{{ result.description
              }}</div>
            </div>

            <div v-if="isIconWithTitleAndDescriptionResult(result)" class="flex items-center">
              <img :src="convertFileSrc(result.icon!!)" class=" object-contain icon" :class="getIconHeightClass()"
                :style="{ filter: getCSSFilterFromHexColor(result.icon_color) }">
              <div class="flex-grow flex flex-col ml-2 justify-center">
                <div class="font-medium min-w-0 oneLineText">
                  {{ result.title }}
                </div>
                <div class=" subtext text-xs min-w-0 oneLineText">
                  {{ result.description }}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: v-bind(tertiaryBackgroundColor);
}

::-webkit-scrollbar-thumb {
  background: v-bind(accentColor);
  border-radius: 48px;
}

.text {
  color: v-bind(textColor);
  line-height: 1em;
}

.subtext {
  color: v-bind(secondaryTextColor);
}

.maxHeight {

  max-height: v-bind(resultsBoxHeight);
}

.selectedResult {
  background-color: v-bind(secondaryBackgroundColor);
  border-radius: v-bind(roundnessLevel);
}

.mainBox {
  width: 780px;
  background-color: v-bind(backgroundColor);
  border: solid v-bind(borderWidth) v-bind(accentColor);
  border-radius: v-bind(roundnessLevel);
}


.searchBox {
  padding-left: 16px;
  padding-right: 16px;
  width: 780px;
  overflow: hidden;
}

.splitSearchBox {
  padding-left: 16px;
  padding-right: 16px;
  width: 780px;
  overflow: hidden;
  background-color: v-bind(backgroundColor);
  border: solid v-bind(borderWidth) v-bind(accentColor);
  border-radius: v-bind(roundnessLevel);
}

.resultsBox {
  overflow-y: auto;
  overflow-x: hidden;
  padding: 8px;
}

.splitResultsBox {
  overflow-y: auto;
  overflow-x: hidden;
  padding: 8px;
  background-color: v-bind(backgroundColor);
  border: solid v-bind(borderWidth) v-bind(accentColor);
  border-radius: v-bind(roundnessLevel);
}

.smallResult {
  height: 50px;
}

.mediumResult {
  height: 60px;
}

.largeResult {
  height: 70px;
}

.smallIcon {
  height: 30px;
  width: 30px;
}

.mediumIcon {
  height: 35px;
  width: 35px;
}

.largeIcon {
  height: 40px;
  width: 40px;
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

.background {
  background-color: v-bind(backgroundColor);
}

.stroke {
  fill: none;
  stroke: v-bind(accentColor);
  stroke-width: 2;
}
</style>