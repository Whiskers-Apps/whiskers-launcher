<script setup lang="ts">
import { PropType, onMounted, ref } from "vue";
import { SearchEngine, ViewModel } from "./ViewModel";
import PrimaryButton from "@components/PrimaryButton.vue";
import { getIconUrl } from "@/utils";
import { getHexCssFilter } from "@/utils";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

const props = defineProps<{ vm: ViewModel }>();

const accentPrimary = ref(props.vm.settings!!.theme.accent_primary);
const backgroundSecondary = ref(props.vm.settings!!.theme.background_secondary);
const backgroundTertiary = ref(props.vm.settings!!.theme.background_tertiary);

onMounted(async()=>{
  await listen("load-theme", (_event)=>{
    accentPrimary.value = props.vm.settings!!.theme.accent_primary;
    backgroundSecondary.value = props.vm.settings!!.theme.background_secondary;
    backgroundTertiary.value = props.vm.settings!!.theme.background_tertiary;
  });
});

function canTintIcon(searchEngine: SearchEngine): boolean {
  return searchEngine.tint_icon || searchEngine.icon_path === null;
}
</script>

<template>
  <div>
    <div class="flex justify-end items-center">
      <PrimaryButton text="Add" :theme="vm.settings!!.theme" @click="vm.addSearchEngine()" />
    </div>
    <div class="mt-2">
      <div
        v-for="(searchEngine, index) in vm.settings!!.search_engines"
        :key="`search-engine-${index}`"
        class="flex search-engine-card p-4 rounded-2xl mb-1 items-center"
      >
        <img
          :src="
            searchEngine.icon_path
              ? convertFileSrc(searchEngine.icon_path)
              : getIconUrl('search.svg')
          "
          :style="{
            filter: canTintIcon(searchEngine) ? getHexCssFilter(accentPrimary) : 'none',
          }"
          width="40"
        />

        <div class="ml-2">{{ searchEngine.name }}</div>

        <img
          v-if="searchEngine.default"
          :src="getIconUrl('check-round.svg')"
          :style="{
            filter: getHexCssFilter(accentPrimary),
          }"
          width="32"
          class="ml-2"
        />
        <div class="flex-grow"></div>
        <div class="p-1 pr-2 pl-2 ml-2 keyword rounded-full">
          {{ searchEngine.keyword }}
        </div>
        <button
          class="hover-background-tertiary p-1 pr-2 pl-2 ml-2 rounded-full"
          @click="vm.editSearchEngine(index)"
        >
          <img
            :src="getIconUrl('pencil.svg')"
            :style="{ filter: getHexCssFilter(accentPrimary) }"
            width="32"
          />
        </button>
        <button
          class="hover-background-tertiary p-1 pr-2 pl-2 ml-2 rounded-full"
          @click="vm.toggleSearchEngineMenu(index)"
        >
          <img
            :src="getIconUrl('three-dots.svg')"
            :style="{ filter: getHexCssFilter(accentPrimary) }"
            width="32"
          />
        </button>
        <div class="menu">
          <div class="menu-content" :id="`search-engine-menu-${index}`">
            <button
              class="p-2 pl-3 pr-3 hover:opacity-80 text-start w-full whitespace-nowrap"
              @click="vm.makeDefaultSearchEngine(index)"
            >
              Make Default
            </button>
            <button
              class="p-2 pl-3 pr-3 hover:opacity-80 text-start w-full whitespace-nowrap"
              @click="vm.deleteSearchEngine(index)"
            >
              Delete
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.search-engine-card {
  background-color: v-bind(backgroundSecondary);
}

.keyword,
.hover-background-tertiary:hover {
  background-color: v-bind(backgroundTertiary);
}

.menu {
  position: relative;
  display: inline-block;
}

.menu-content {
  display: none;
  position: absolute;
  background-color: v-bind(backgroundTertiary);
  z-index: 9999;
  right: 0;
  border: 2px solid v-bind(accentPrimary);
  border-radius: 8px;
  margin-top: 16px;
}
</style>
