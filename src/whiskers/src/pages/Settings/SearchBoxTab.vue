<script setup lang="ts">
import SwitchSetting from "@/components/SwitchSetting.vue";
import { PropType, onMounted, ref } from "vue";
import { ViewModel } from "./ViewModel";
import RadioButton from "@/components/RadioButton.vue";
import SliderSetting from "@/components/SliderSetting.vue";
import { listen } from "@tauri-apps/api/event";

const props = defineProps<{ vm: ViewModel }>();

const accentPrimary = ref(props.vm.settings!!.theme.accent_primary);
const backgroundTertiary = ref(props.vm.settings!!.theme.background_tertiary);

onMounted(async () => {
  await listen("load-theme", (_event) => {
    accentPrimary.value = props.vm.settings!!.theme.accent_primary;
    backgroundTertiary.value = props.vm.settings!!.theme.background_tertiary;
  });
});
</script>

<template>
  <div>
    <SwitchSetting
      title="Search Icon"
      description="If enabled, it shows a search icon"
      :checked="vm.settings!!.show_search_icon"
      @update-checked="vm.updateShowSearchIcon($event)"
      :theme="vm.settings!!.theme"
    />

    <div class="divider mt-2 mb-2"></div>

    <SwitchSetting
      title="Settings Icon"
      description="If enabled, it shows a settings icon"
      :checked="vm.settings!!.show_settings_icon"
      @update-checked="vm.updateShowSettingsIcon($event)"
      :theme="vm.settings!!.theme"
    />

    <div class="divider mt-2 mb-2"></div>

    <SwitchSetting
      title="Placeholder"
      description="If enabled, it shows a placeholder text"
      :checked="vm.settings!!.show_placeholder"
      @update-checked="vm.updateShowPlaceholder($event)"
      :theme="vm.settings!!.theme"
    />

    <div class="divider mt-2 mb-2"></div>

    <div class="ml-2 mr-2">
      <div class="title">Layout</div>
      <div>Select the desired layout for the search box</div>
      <div class="grid grid-cols-2 gap-2">
        <div
          class="background-secondary p-4 rounded-2xl cursor-pointer layout-card"
          @click="vm.updateLayout('highlight')"
        >
          <div class="p-2 background-main rounded-2xl h-[200px] layout-border">
            <div class="background-secondary p-3 rounded-full"></div>
          </div>
          <div class="flex flex-col items-center">
            <div class="layout-divider"></div>
            <RadioButton
              text="Highlight"
              :checked="vm.settings!!.layout === 'highlight'"
              :theme="vm.settings!!.theme"
            />
          </div>
        </div>

        <div
          class="background-secondary p-4 rounded-2xl cursor-pointer layout-card"
          @click="vm.updateLayout('highlight_split')"
        >
          <div class="h-[200px] flex flex-col">
            <div class="layout-border background-main p-2 rounded-full">
              <div class="background-secondary p-3 rounded-full"></div>
            </div>
            <div class="mt-2 layout-border flex-grow rounded-2xl background-main"></div>
          </div>
          <div class="flex flex-col items-center">
            <div class="layout-divider"></div>
            <RadioButton
              text="Highlight Split"
              :checked="vm.settings!!.layout === 'highlight_split'"
              :theme="vm.settings!!.theme"
            />
          </div>
        </div>

        <div
          class="background-secondary p-4 rounded-2xl cursor-pointer layout-card"
          @click="vm.updateLayout('modern')"
        >
          <div class="p-2 background-main rounded-2xl h-[200px] layout-border"></div>
          <div class="flex flex-col items-center">
            <div class="layout-divider"></div>
            <RadioButton
              text="Modern"
              :checked="vm.settings!!.layout === 'modern'"
              :theme="vm.settings!!.theme"
            />
          </div>
        </div>

        <div
          class="background-secondary p-4 rounded-2xl cursor-pointer layout-card"
          @click="vm.updateLayout('modern_split')"
        >
          <div class="h-[200px] flex flex-col">
            <div class="layout-border background-main p-2 rounded-full">
              <div class="p-3 rounded-full"></div>
            </div>
            <div class="mt-2 layout-border flex-grow rounded-2xl background-main"></div>
          </div>
          <div class="flex flex-col items-center">
            <div class="layout-divider"></div>
            <RadioButton
              text="Modern Split"
              :checked="vm.settings!!.layout === 'modern_split'"
              :theme="vm.settings!!.theme"
            />
          </div>
        </div>
      </div>
    </div>

    <div class="divider mt-2 mb-2"></div>

    <SliderSetting
      :title="`Border Radius (${vm.settings!!.border_radius})`"
      description="Applies a border radius to the search box"
      :min="0"
      :max="48"
      :step="1"
      :value="vm.settings!!.border_radius"
      :checked="vm.settings!!.border_radius"
      :theme="vm.settings!!.theme"
      @update-value="vm.updateBorderRadius($event)"
    />

    <div class="divider mt-2 mb-2"></div>

    <SliderSetting
      :title="`Border Width (${vm.settings!!.border_width})`"
      description="Applies a border width to the search box"
      :min="0"
      :max="14"
      :step="1"
      :value="vm.settings!!.border_width"
      :checked="vm.settings!!.border_width"
      :theme="vm.settings!!.theme"
      @update-value="vm.updateBorderWidth($event)"
    />
  </div>
</template>

<style scoped>
.layout-border {
  border: 2px solid v-bind(accentPrimary);
}

.layout-divider {
  height: 6px;
  width: 60px;
  background-color: v-bind(accentPrimary);
  margin: 16px 0 16px 0px;
  border-radius: 48px;
}

.layout-card:hover {
  background-color: v-bind(backgroundTertiary);
}
</style>
