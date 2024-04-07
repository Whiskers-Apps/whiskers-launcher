<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { ViewModel } from "./ViewModel";
import { getHexCssFilter, getIconUrl, getScaledPixels } from "@/utils";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { PhysicalSize, appWindow, currentMonitor } from "@tauri-apps/api/window";

const vm = ref<ViewModel>(new ViewModel());

// =====================================
// Colors
// =====================================
const backgroundMain = ref("");
const backgroundSecondary = ref("");
const backgroundTertiary = ref("");
const accentPrimary = ref("");
const accentPrimaryFilter = ref("");
const textOnBackground = ref("");
const textOnPrimary = ref("");

// ======================================
// Sizes
// ======================================
const searchBarWidth = ref("800px");
const dividerSize = ref("2px");
const smallSize = ref("8px");
const mediumSize = ref("12px");
const largeSize = ref("16px");
const extraLargeSize = ref("20px");
const borderRadius = ref("32px");
const borderWidth = ref("2px");
const overallPadding = ref("8px");
const smallTextSize = ref("12px");
const normalTextSize = ref("16px");
const searchIconSize = ref("30px");
const resultHeight = ref("48px");
const innerResultHeight = ref("40px");

onMounted(async () => {
  appWindow.center();

  let monitor = await currentMonitor();
  appWindow.setSize(monitor!!.size);

  await vm.value.load();

  const settings = vm.value.settings!!;
  const fractionScaling = settings.fractional_scaling;

  const theme = settings.theme;
  backgroundMain.value = theme.background_main;
  backgroundSecondary.value = theme.background_secondary;
  backgroundTertiary.value = theme.background_tertiary;
  accentPrimary.value = theme.accent_primary;
  accentPrimaryFilter.value = getHexCssFilter(theme.accent_primary);
  textOnBackground.value = theme.text_on_background;
  textOnPrimary.value = theme.text_on_primary;

  dividerSize.value = getScaledPixels(fractionScaling, 2);
  searchBarWidth.value = getScaledPixels(fractionScaling, 800);
  smallSize.value = getScaledPixels(fractionScaling, 6);
  mediumSize.value = getScaledPixels(fractionScaling, 8);
  largeSize.value = getScaledPixels(fractionScaling, 10);
  extraLargeSize.value = getScaledPixels(fractionScaling, 20);
  borderRadius.value = getScaledPixels(fractionScaling, settings.border_radius);
  borderWidth.value = getScaledPixels(fractionScaling, settings.border_width);

  if (settings.density === "small") {
    overallPadding.value = smallSize.value;
    smallTextSize.value = getScaledPixels(fractionScaling, 12);
    normalTextSize.value = getScaledPixels(fractionScaling, 14);
    resultHeight.value = getScaledPixels(fractionScaling, 32 + 6);
    innerResultHeight.value = getScaledPixels(fractionScaling, 32);
    searchIconSize.value = getScaledPixels(fractionScaling, 20);
  } else if (settings.density === "large") {
    overallPadding.value = largeSize.value;
    smallTextSize.value = getScaledPixels(fractionScaling, 16);
    normalTextSize.value = getScaledPixels(fractionScaling, 18);
    resultHeight.value = getScaledPixels(fractionScaling, 48 + 10);
    innerResultHeight.value = getScaledPixels(fractionScaling, 32);
    searchIconSize.value = getScaledPixels(fractionScaling, 26);
  } else {
    overallPadding.value = mediumSize.value;
    smallTextSize.value = getScaledPixels(fractionScaling, 14);
    normalTextSize.value = getScaledPixels(fractionScaling, 16);
    resultHeight.value = getScaledPixels(fractionScaling, 40 + 8);
    innerResultHeight.value = getScaledPixels(fractionScaling, 32);
    searchIconSize.value = getScaledPixels(fractionScaling, 24);
  }

  vm.value.hasLoaded = true;
});

document.addEventListener("keydown", (event) => {
  switch (event.key) {
    case "ArrowDown": {
      event.preventDefault();
      vm.value.onArrowDownPress();
      break;
    }

    case "ArrowUp": {
      event.preventDefault();
      vm.value.onArrowUpPress();
      break;
    }

    case "Escape": {
      appWindow.close();
      break;
    }

    case "Enter": {
      vm.value.runAction();
      break;
    }
  }

  if (event.ctrlKey && event.key === "s") {
    vm.value.openSettings();
  }

  if (event.altKey && ["1", "2", "3", "4", "5", "6", "7", "8"].includes(event.key)) {
    vm.value.selectAltResult(event.key)
  }
});
</script>

<template>
  <div
    v-if="vm.hasLoaded"
    class="flex justify-center main h-screen"
    @click="
      if (vm.settings!!.hide_on_blur) {
        appWindow.close();
      }
    "
  >
    <div class="p-overall mt-[140px]">
      <div :class="!vm.isSplitLayout() ? 'search-box p-overall' : ''">
        <div :class="vm.isSplitLayout() ? 'search-box p-overall' : ''">
          <div class="search-div" :style="{ backgroundColor: vm.getSearchInputBackground() }">
            <div class="flex p-overall items-center">
              <div class="p-1" v-if="vm.settings!!.show_search_icon">
                <img class="icon" :src="getIconUrl('search.svg')"/>
              </div>

              <input
                autofocus
                class="search-input normal-text flex-grow"
                :placeholder="vm.getInputPlaceholder()"
                :style="{ backgroundColor: vm.getSearchInputBackground() }"
                v-model="vm.typedText"
                @input="vm.search()"
                @click="$event.stopPropagation()"
              />

              <button
                class="p-1 hover-background-tertiary rounded-full"
                @click="vm.openSettings($event)"
                v-if="vm.settings!!.show_settings_icon"
              >
                <img class="icon" :src="getIconUrl('settings.svg')" />
              </button>
            </div>
          </div>
        </div>
        <div
          id="results-div"
          class="mt-large overflow-auto"
          :class="vm.isSplitLayout() ? 'search-box p-overall' : ''"
          :hidden="vm.results.length === 0"
        >
          <div :style="{ height: vm.resultsHeight }">
            <div
              v-for="(result, index) in vm.displayedResults"
              :key="`result-${index}`"
              :id="`result-${index}`"
              class="p-overall one-line-text"
              :class="
                vm.isHighlightLayoutType() && vm.selectedIndex === index
                  ? 'highlight-background'
                  : ''
              "
            >
              <div
                class="result cursor-pointer p-overall flex items-center one-line-text"
                @click="
                  $event.stopPropagation();
                  vm.runAction();
                "
                @mouseover="vm.selectedIndex = index"
              >
                <div
                  v-if="['Text', 'TitleAndText'].includes(result.type)"
                  class="flex items-center one-line-text w-full"
                >
                  <img
                    v-if="result.icon === ''"
                    class="app-icon primary-filter"
                    :src="getIconUrl('default-app-icon.svg')"
                  />

                  <img
                    v-else-if="result.icon !== null"
                    :src="
                      result.icon ? convertFileSrc(result.icon) : getIconUrl('default-app-icon')
                    "
                    class="app-icon"
                    :style="{
                      filter: result.tint_icon
                        ? result.tint_color
                          ? getHexCssFilter(result.tint_color)
                          : accentPrimaryFilter
                        : 'none',
                    }"
                  />

                  <div
                    class="flex flex-col justify-center ml-medium one-line-text flex-grow"
                    :class="vm.selectedIndex === index ? 'highlight-text' : ''"
                  >
                    <div class="normal-text one-line-text">
                      {{ result.title }}
                    </div>
                    <div
                      class="one-line-text"
                      :class="result.type === 'Text' ? 'normal-text' : 'small-text'"
                    >
                      {{ result.text }}
                    </div>
                  </div>

                  <div v-if="vm.settings!!.show_alt_hint" class="ml-4 primary-text">Alt + {{ index + 1 }}</div>
                </div>

                <div v-if="result.type === 'Divider'" class="w-full">
                  <div class="result-divider"></div>
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
.main {
  color: v-bind(textOnBackground);
}

.search-box {
  background-color: v-bind(backgroundMain);
  width: v-bind(searchBarWidth);
  border-radius: v-bind(borderRadius);
  border: v-bind(borderWidth) solid v-bind(accentPrimary);
}

.primary-filter {
  filter: v-bind(accentPrimaryFilter);
}

.p-overall {
  padding: v-bind(overallPadding);
}

.p-small {
  padding: v-bind(smallSize);
}

.p-medium {
  padding: v-bind(mediumSize);
}

.ml-medium {
  margin-left: v-bind(mediumSize);
}

.mt-large {
  margin-top: v-bind(largeSize);
}

.icon {
  filter: v-bind(accentPrimaryFilter);
  width: v-bind(searchIconSize);
}

.app-icon {
  width: v-bind(innerResultHeight);
}

.search-div {
  border-radius: v-bind(borderRadius);
  overflow: hidden;
}

.normal-text {
  font-size: v-bind(normalTextSize);
}

.title-text {
  font-size: v-bind(smallTextSize);
}

.small-text {
  font-size: v-bind(smallTextSize);
}

.search-input:focus {
  outline: none;
}

.search-input::placeholder {
  color: v-bind(textOnBackground);
}

.hover-background-tertiary:hover {
  background-color: v-bind(backgroundTertiary);
}

.highlight-background {
  background-color: v-bind(backgroundSecondary);
  border-radius: v-bind(borderRadius);
}

.highlight-text {
  color: v-bind(accentPrimary);
  font-weight: 700;
}

.divider-result {
  height: v-bind(resultHeight);
}

.result {
  height: v-bind(resultHeight);
  max-height: v-bind(resultHeight);
}
.result-divider {
  height: v-bind(dividerSize);
  width: 100%;
  background-color: v-bind(accentPrimary);
  border-radius: 48px;
}

.primary-text {
  color: v-bind(accentPrimary);
}
</style>
