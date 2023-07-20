<script setup lang="ts">
import { appWindow, PhysicalSize, WebviewWindow } from "@tauri-apps/api/window"
import { invoke } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { onMounted, ref, watch } from "vue";
import SearchSVG from "../../assets/icons/search.svg";
import SettingsSVG from "../../assets/icons/settings.svg";
import { getSettings, getRoundnessInPixels } from "../../pages/Settings/Settings";
import { listen } from "@tauri-apps/api/event"
import { SimpleKlResult, TextResult } from "../../data"
import { hexToCSSFilter } from "hex-to-css-filter"

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
const results = ref<SimpleKlResult[]>([]);
const resultsLimit = ref(0)
const searchBoxHeight = ref("70px");
const selectedIndex = ref(0);
const resultsRef = ref([]);


function openSettings() {
  new WebviewWindow('settings', {
    url: 'settings',
    title: "Settings",
    width: 1100
  })

  appWindow.close()
}

function getCSSFilterFromHexColor(hexColor?: string): string {

  if (hexColor !== null) {

    let color = hexColor!!;

    if (hexColor === "accent") {
      color = accentColor.value
    }

    let filter = hexToCSSFilter(color);

    console.log(filter)

    return filter.filter.replace(";", "")
  }

  return "none"
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
  resultsLimit.value = settings.general.limit;

  searchRef.value.focus();

  focus.value = await listen("focus_box", (event) => {
    appWindow.show();
    appWindow.setFocus();
    searchRef.value.focus();
  })
})

document.addEventListener('keydown', function (event) {

  if (event.key === "ArrowDown") {

    event.preventDefault(); //Prevents cursor from changing position

    if (selectedIndex.value < results.value.length - 1) {
      selectedIndex.value = selectedIndex.value + 1;
      (resultsRef.value[selectedIndex.value - 1] as HTMLDivElement).scrollIntoView({ behavior: 'smooth' });
    }
  }

  if (event.key === "ArrowUp") {

    event.preventDefault(); //Prevents cursor from changing position

    if (selectedIndex.value > 0) {
      selectedIndex.value = selectedIndex.value - 1;
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
    appWindow.close()
  }

  if (event.key === "Enter") {
    runAction();
  }
});

async function runAction() {
  if (results.value.length > 0) {

    var actionJson = "";
    var actionType = "";
    var result = results.value[selectedIndex.value];


    if (result.Text !== undefined) {
      let action = result.Text.action;

      if (action.OpenApp !== undefined) {
        actionJson = JSON.stringify(action.OpenApp);
        actionType = "OpenApp"
      }

      if (action.OpenInBrowser !== undefined) {
        actionJson = JSON.stringify(action.OpenInBrowser);
        actionType = "OpenInBrowser"
      }

      if (action.CopyToClipboard !== undefined) {
        actionJson = JSON.stringify(action.CopyToClipboard);
        actionType = "CopyToClipboard"
      }

      if (action.ExtensionAction !== undefined) {
        actionJson = JSON.stringify(action.ExtensionAction);
        actionType = "ExtensionAction"
      }

      if (action.DoNothingAction !== undefined) {
        actionType = "DoNothingAction"
      }
    }

    if (result.IconWithText !== undefined) {
      let action = result.IconWithText.action;

      if (action.OpenApp !== undefined) {
        actionJson = JSON.stringify(action.OpenApp);
        actionType = "OpenApp"
      }

      if (action.OpenInBrowser !== undefined) {
        actionJson = JSON.stringify(action.OpenInBrowser);
        actionType = "OpenInBrowser"
      }

      if (action.CopyToClipboard !== undefined) {
        actionJson = JSON.stringify(action.CopyToClipboard);
        actionType = "CopyToClipboard"
      }

      if (action.ExtensionAction !== undefined) {
        actionJson = JSON.stringify(action.ExtensionAction);
        actionType = "ExtensionAction"
      }

      if (action.DoNothingAction !== undefined) {
        actionType = "DoNothingAction"
      }
    }

    if (result.TitleAndDescription !== undefined) {
      let action = result.TitleAndDescription.action;

      if (action.OpenApp !== undefined) {
        actionJson = JSON.stringify(action.OpenApp);
        actionType = "OpenApp"
      }

      if (action.OpenInBrowser !== undefined) {
        actionJson = JSON.stringify(action.OpenInBrowser);
        actionType = "OpenInBrowser"
      }

      if (action.CopyToClipboard !== undefined) {
        actionJson = JSON.stringify(action.CopyToClipboard);
        actionType = "CopyToClipboard"
      }

      if (action.ExtensionAction !== undefined) {
        actionJson = JSON.stringify(action.ExtensionAction);
        actionType = "ExtensionAction"
      }

      if (action.DoNothingAction !== undefined) {
        actionType = "DoNothingAction"
      }
    }

    if (result.IconWithTitleAndDescription !== undefined) {
      let action = result.IconWithTitleAndDescription.action;

      if (action.OpenApp !== undefined) {
        actionJson = JSON.stringify(action.OpenApp);
        actionType = "OpenApp"
      }

      if (action.OpenInBrowser !== undefined) {
        actionJson = JSON.stringify(action.OpenInBrowser);
        actionType = "OpenInBrowser"
      }

      if (action.CopyToClipboard !== undefined) {
        actionJson = JSON.stringify(action.CopyToClipboard);
        actionType = "CopyToClipboard"
      }

      if (action.ExtensionAction !== undefined) {
        actionJson = JSON.stringify(action.ExtensionAction);
        actionType = "ExtensionAction"
      }

      if (action.DoNothingAction !== undefined) {
        actionType = "DoNothingAction"
      }
    }

    if (actionType !== "DoNothingAction") {
      invoke("run_action", { action_json: actionJson, action_type: actionType })
      appWindow.close()
    }
  }
}

watch(searchText, async (_newText, _oldText) => {

  if (searchText.value.trim() === "") {

    results.value = [];
    appWindow.setSize(new PhysicalSize(800, 70));
    searchBoxHeight.value = `70px`
  } else {

    results.value = JSON.parse(await invoke("get_results", { search_text: searchText.value }));
    var newHeight = 70;

    if (results.value.length > resultsLimit.value) {
      newHeight = newHeight + (resultsLimit.value * 70);
    } else {
      newHeight = newHeight + results.value.length * 70;
    }

    appWindow.setSize(new PhysicalSize(800, newHeight));
    searchBoxHeight.value = `${newHeight}px`
  }

  selectedIndex.value = 0;
})




</script>

<template>
  <div class="items-center justify-center h-screen w-screen max-h-screen text maxHeight">
    <div class="searchBox pt-2 pb-2 pl-4 pr-4 flex flex-col justify-center">
      <div class="flex  items-center">
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
      <div v-if="results.length > 0" class="overflow-auto mt-2">
        <div v-for="(result, index) in results" ref="resultsRef">
          <div :ref="`result-${index}`" class="h-[70px] p-2 flex overflow-hidden"
            :class="index === selectedIndex ? 'selectedResult' : ''">

            <div v-if="result.Text !== undefined" class="flex items-center">
              <div>{{ result.Text.text }}</div>
            </div>

            <div v-if="result.IconWithText !== undefined" class="flex items-center">
              <img :src="convertFileSrc(result.IconWithText.icon)" class="h-[35px] w-[35px] object-contain icon"
                :style="{ filter: getCSSFilterFromHexColor(result.IconWithText!!.icon_color) }">
              <div class="text-lg ml-2 flex-grow">{{ result.IconWithText.text }}</div>
            </div>

            <div v-if="result.TitleAndDescription !== undefined" class="flex flex-col justify-center p-2">
              <div class="text-[16px] font-bold text-ellipsis whitespace-nowrap">{{ result.TitleAndDescription!!.title }}
              </div>
              <div class="text-[15px] subtext text-ellipsis whitespace-nowrap">{{ result.TitleAndDescription!!.description
              }}</div>
            </div>

            <div v-if="result.IconWithTitleAndDescription !== undefined" class="flex items-center">
              <img :src="convertFileSrc(result.IconWithTitleAndDescription.icon)"
                class="h-[35px] w-[35px] object-contain icon"
                :style="{ filter: getCSSFilterFromHexColor(result.IconWithTitleAndDescription!!.icon_color) }">
              <div class="flex-grow flex flex-col ml-2">
                <div class="text-[16px] font-bold text-ellipsis whitespace-nowrap">{{
                  result.IconWithTitleAndDescription!!.title
                }}
                </div>
                <div class="text-[15px] subtext text-ellipsis whitespace-nowrap">{{
                  result.IconWithTitleAndDescription!!.description
                }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.text {
  color: v-bind(textColor)
}

.subtext {
  color: v-bind(secondaryTextColor);
}

.maxHeight {

  max-height: v-bind(searchBoxHeight);
}

.selectedResult {
  background-color: v-bind(secondaryBackgroundColor);
  border-radius: v-bind(roundnessLevel);
}

.searchBox {
  background-color: v-bind(backgroundColor);
  border: solid v-bind(borderWidth) v-bind(accentColor);
  font-size: 1.1rem;
  border-radius: v-bind(roundnessLevel);
  min-height: 65px;
  max-height: v-bind(searchBoxHeight);
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