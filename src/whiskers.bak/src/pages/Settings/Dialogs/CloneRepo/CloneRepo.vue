<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { onMounted, ref } from "vue";
import { getSettings } from "@/utils";
import InputSetting from "@/components/InputSetting.vue";
import { Theme } from "@pages/Settings/ViewModel";
import PrimaryButton from "@/components/PrimaryButton.vue";
import { emit } from "@tauri-apps/api/event";
import { CloneExtensionPayload } from "@/DialogPayloads";
import { appWindow } from "@tauri-apps/api/window";

const hasLoaded = ref(false);
const theme = ref<Theme>();
const backgroundMain = ref("");
const textOnBackground = ref("");
const url = ref("");

onMounted(async () => {
  let settings = await getSettings();
  theme.value = settings.theme;

  backgroundMain.value = settings.theme.background_main;
  textOnBackground.value = settings.theme.text_on_background;
  hasLoaded.value = true;
});

function clone() {
  let payload: CloneExtensionPayload = { url: url.value };
  emit("clone-repo", payload);
  appWindow.close();
}
</script>

<template>
  <div class="main h-screen max-h-screen p-4 flex flex-col" v-if="hasLoaded">
    <InputSetting
      title="Clone Extension"
      description="Insert the url of the repo containg the extension"
      placeholder="Url"
      :value="url"
      :theme="theme!!"
      @update-value="url = $event"
    />
    <div class="flex justify-end mt-4 mr-2">
      <PrimaryButton
        :disabled="url.trim().length === 0"
        text="Clone"
        :theme="theme!!"
        @click="clone()"
      />
    </div>
  </div>
</template>

<style>
.main {
  background-color: v-bind(backgroundMain);
  color: v-bind(textOnBackground);
}
</style>
