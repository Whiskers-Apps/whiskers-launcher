<script setup lang="ts">
import { appWindow, PhysicalSize, WebviewWindow } from "@tauri-apps/api/window"
import { invoke } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { onMounted, ref, watch } from "vue";
import SearchSVG from "../../assets/icons/search.svg";
import SettingsSVG from "../../assets/icons/settings.svg";
import { getSettings, getRoundnessInPixels, getTheme } from "@pages/Settings/Settings";
import { SimpleKlResult, OpenAppAction } from "@/data"
import { hexToCSSFilter } from "hex-to-css-filter"
import { isCopyToClipboardAction, isDoNothingAction, isExtensionAction, isOpenAppAction, isOpenInBrowserAction, isIconWithTitleAndDescriptionResult, isTitleAndDescriptionResult, isIconWithTextResult, isTextResult } from "@pages/Search/SearchVM"

const showSearchIcon = ref();
const showSettingsIcon = ref();
const roundnessLevel = ref();
const borderWidth = ref();
const searchText = ref("")
const backgroundColor = ref("")
const secondaryBackgroundColor = ref("")
const tertiaryBackgroundColor = ref("")
const accentColor = ref("")
const textColor = ref("")
const secondaryTextColor = ref("")
const searchRef = ref();
const results = ref<SimpleKlResult[]>([]);
const resultsLimit = ref(0)
const resultsBoxHeight = ref("70px");
const selectedIndex = ref(0);
const resultsRef = ref([]);


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


onMounted(async () => {

  searchRef.value.focus();

  let settings = await getSettings();

  roundnessLevel.value = getRoundnessInPixels(settings.search_box.roundness);
  borderWidth.value = `${settings.search_box.border_width}px`
  showSearchIcon.value = settings.search_box.show_search_icon;
  showSettingsIcon.value = settings.search_box.show_settings_icon;
  resultsLimit.value = settings.general.limit;

  let theme = await getTheme();

  backgroundColor.value = theme.background;
  secondaryBackgroundColor.value = theme.secondary_background;
  accentColor.value = theme.accent;
  textColor.value = theme.text;
  secondaryTextColor.value = theme.secondary_text;

})

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
    
    var newHeight = 55;

    if (results.value.length > resultsLimit.value) {
      newHeight = newHeight + (resultsLimit.value * 55);
    } else {
      newHeight = newHeight + results.value.length * 55;
    }

    resultsBoxHeight.value = `${newHeight}px`
    
  }

  selectedIndex.value = 0;
})

</script>

<template>
  <div class="items-center flex flex-col h-screen w-screen max-h-screen text maxHeight">
    <div class="mainBox">
      <div class="flex items-center searchBox">
        <div v-if="showSearchIcon" class="mr-2">
          <SearchSVG class="w-5 h-5 stroke" />
        </div>
        <div class="flex-grow">
          <input ref="searchRef" class="w-full background outline-none placeholder" placeholder="Search"
            v-model="searchText" />
        </div>
        <button v-if="showSettingsIcon" class="ml-2 secondaryHover rounded-full" @click="openSettings">
          <SettingsSVG class="w-5 h-5 stroke" />
        </button>
      </div>
      <div v-if="results.length > 0" class="resultsBox">
        <div v-for="(result, index) in results" ref="resultsRef">
          <div :ref="`result-${index}`" class="h-[55px] p-2 flex overflow-hidden"
            :class="index === selectedIndex ? 'selectedResult' : ''">

            <div v-if="isTextResult(result)" class="flex items-center">
              <div>{{ result.text }}</div>
            </div>

            <div v-if="isIconWithTextResult(result)" class="flex items-center">
              <img :src="convertFileSrc(result.icon!!)" class="h-[30px] w-[30px] object-contain icon"
                :style="{ filter: getCSSFilterFromHexColor(result.icon_color) }">
              <div class="text-lg ml-2 flex-grow">{{ result.text }}</div>
            </div>

            <div v-if="isTitleAndDescriptionResult(result)" class="flex flex-col justify-center p-2">
              <div class="text-[16px] font-bold text-ellipsis whitespace-nowrap">{{ result.title }}
              </div>
              <div class="text-[15px] subtext text-ellipsis whitespace-nowrap">{{ result.description
              }}</div>
            </div>

            <div v-if="isIconWithTitleAndDescriptionResult(result)" class="flex items-center">
              <img :src="convertFileSrc(result.icon!!)" class="h-[30px] w-[30px] object-contain icon"
                :style="{ filter: getCSSFilterFromHexColor(result.icon_color) }">
              <div class="flex-grow flex flex-col ml-2">
                <div class="text-[16px] font-bold text-ellipsis whitespace-nowrap">
                  {{ result.title }}
                </div>
                <div class="text-[15px] subtext text-ellipsis whitespace-nowrap">
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
  color: v-bind(textColor)
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
  background-color: v-bind(backgroundColor);
  border: solid v-bind(borderWidth) v-bind(accentColor);
  border-radius: v-bind(roundnessLevel);
  width: 700px;
}

.searchBox {
  padding: 12px;
  width: 700px;
}

.resultsBox {
  max-height: v-bind(resultsBoxHeight);
  overflow-y: auto;
  overflow-x: hidden;
  padding: 12px;
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