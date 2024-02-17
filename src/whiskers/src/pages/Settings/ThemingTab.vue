<script setup lang="ts">
import PrimaryButton from "@/components/PrimaryButton.vue";
import { ref } from "vue";
import { ViewModel } from "./ViewModel";
import { WebviewWindow } from "@tauri-apps/api/window";

const props = defineProps<{
  vm: ViewModel;
}>();

const textOnBackground = ref(props.vm.settings?.theme.text_on_background ?? "ERROR!!");

async function openThemeStore() {
  new WebviewWindow("themes-store", {
    url: "themes-store",
    title: "Themes Store",
    width: 1000,
    height: 800,
    center: true,
  });
}
</script>

<template>
  <div>
    <div class="flex">
      <PrimaryButton text="Import" :theme="vm.settings!!.theme" @click="vm.importTheme()" />
      <PrimaryButton
        class="ml-2"
        text="Export"
        :theme="vm.settings!!.theme"
        @click="vm.exportTheme()"
      />
      <PrimaryButton class="ml-2" text="Themes Store" :theme="vm.settings!!.theme" @click="openThemeStore()" />
    </div>
    <div class="title mt-2">Background Colors</div>
    <div class="background-secondary rounded-2xl p-4">
      <div class="flex items-center">
        <div class="flex-grow mr-2">Main</div>
        <input
          :value="vm.settings!!.theme.background_main"
          type="color"
          class="color-input"
          @input="vm.updateBackgroundMain(($event.target as HTMLInputElement).value)"
        />
      </div>
      <div class="divider mt-2 mb-2"></div>

      <div class="flex items-center">
        <div class="flex-grow mr-2">Secondary</div>
        <input
          :value="vm.settings!!.theme.background_secondary"
          type="color"
          class="color-input"
          @input="vm.updateBackgroundSecondary(($event.target as HTMLInputElement).value)"
        />
      </div>
      <div class="divider mt-2 mb-2"></div>
      <div class="flex items-center">
        <div class="flex-grow mr-2">Tertiary</div>
        <input
          :value="vm.settings!!.theme.background_tertiary"
          type="color"
          class="color-input"
          @input="vm.updateBackgroundTertiary(($event.target as HTMLInputElement).value)"
        />
      </div>
    </div>
    <div class="title mt-2">Accent Colors</div>
    <div class="background-secondary rounded-2xl p-4">
      <div class="flex items-center">
        <div class="flex-grow mr-2">Primary</div>
        <input
          :value="vm.settings!!.theme.accent_primary"
          type="color"
          class="color-input"
          @input="vm.updateAccentPrimary(($event.target as HTMLInputElement).value)"
        />
      </div>
      <div class="divider mt-2 mb-2"></div>

      <div class="flex items-center">
        <div class="flex-grow mr-2">Danger</div>
        <input
          :value="vm.settings!!.theme.accent_danger"
          type="color"
          class="color-input"
          @input="vm.updateAccentDanger(($event.target as HTMLInputElement).value)"
        />
      </div>
    </div>
    <div class="title mt-2">Text Colors</div>
    <div class="background-secondary rounded-2xl p-4">
      <div class="flex items-center">
        <div class="flex-grow mr-2">On Background</div>
        <input
          :value="vm.settings!!.theme.text_on_background"
          type="color"
          class="color-input"
          @input="vm.updateTextOnBackground(($event.target as HTMLInputElement).value)"
        />
      </div>
      <div class="divider mt-2 mb-2"></div>

      <div class="flex items-center">
        <div class="flex-grow mr-2">On Primary</div>
        <input
          :value="vm.settings!!.theme.text_on_primary"
          type="color"
          class="color-input"
          @input="vm.updateTextOnPrimary(($event.target as HTMLInputElement).value)"
        />
      </div>
      <div class="divider mt-2 mb-2"></div>

      <div class="flex items-center">
        <div class="flex-grow mr-2">On Danger</div>
        <input
          :value="vm.settings!!.theme.text_on_danger"
          type="color"
          class="color-input"
          @input="vm.updateTextOnDanger(($event.target as HTMLInputElement).value)"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.color-input {
  appearance: none;
  -webkit-appearance: none;
  cursor: pointer;
  width: 80px;
  height: 40px;
  background-color: transparent;
}

.color-input::-webkit-color-swatch {
  border-radius: 48px;
  border: 2px solid v-bind(textOnBackground);
}
</style>
