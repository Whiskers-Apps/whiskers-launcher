<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getSettings } from "@/utils";
import PrimaryButton from "@/components/PrimaryButton.vue";
import { Theme } from "../../ViewModel";
import SecondaryButton from "@/components/SecondaryButton.vue";
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";

const hasLoaded = ref(false);
const theme = ref<Theme>();
const backgroundMain = ref("");
const textOnBackground = ref("");
const accentPrimary = ref("");

onMounted(async () => {
  theme.value = (await getSettings()).theme;

  backgroundMain.value = theme.value.background_main;
  textOnBackground.value = theme.value.text_on_background;
  accentPrimary.value = theme.value.accent_primary;
  hasLoaded.value = true;
});

function cancel(){
  appWindow.close();
}

function confirm(){
  emit("confirm-uninstall-extension");
  appWindow.close();
}
</script>

<template>
  <div class="main h-screen max-h-screen flex flex-col justify-center p-4" v-if="hasLoaded">
    <div class="title">Uninstall Extension</div>
    <div>Are you sure you want to uninstall the extension?</div>
    <div class="flex justify-end mt-4">
      <SecondaryButton class="mr-2" text="Cancel" :theme="theme!!" @click="cancel()"/>
      <PrimaryButton text="Uninstall" :theme="theme!!" :danger="true" @click="confirm()"/>
    </div>
  </div>
</template>

<style scoped>
.main {
  background-color: v-bind(backgroundMain);
  color: v-bind(textOnBackground);
}

.title {
  color: v-bind(accentPrimary);
  font-size: 24px;
  font-weight: 700;
}
</style>
