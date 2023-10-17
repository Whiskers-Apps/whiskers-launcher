<script setup lang="ts">
import Logo from "@images/icon.png";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import axios from "axios";
import { ref } from "vue";

const installListener = ref();
const disableButtons = ref(false);
const infoMessage = ref("");

function install() {

  infoMessage.value = "Getting Latest Release ...";

  axios.get("https://api.github.com/repos/lighttigerxiv/simple-keyboard-launcher/releases/latest", {
    headers: {
      "User-Agent": "Simple Keyboard Launcher Manager - Tauri"
    }
  }).then(response => {

    if (response.status === 200) {

      disableButtons.value = true;

      response.data.assets.forEach((asset: any) => {
        if (asset.name.includes("Linux")) {


          invoke("install", { url: asset.browser_download_url, tag: response.data.tag_name });

          installListener.value = listen<{ message: string, finished: boolean }>("install", (event) => {
            infoMessage.value = event.payload.message;
            
            if (event.payload.finished) {
              disableButtons.value = false;
            }
          });
        }
      });
    }

  }).catch(error => {
    console.error(error);
  });
}
</script>

<template>
  <div class="p-6 bg-background h-screen max-h-screen overflow-auto text-text flex flex-col justify-center">
    <div class="flex items-center justify-center">
      <img :src="Logo" class="h-14" />
      <div class="ml-4 text-3xl font-medium">Simple KL Manager</div>
    </div>
    <div class="flex mt-6">
      <button class="flex-grow button bg-accent text-on-accent disabled:bg-mantle disabled:text-text" @click="install()"
        :disabled="disableButtons">
        Install
      </button>
      <button class="flex-grow button bg-danger text-on-danger disabled:bg-mantle ml-6 disabled:text-text"
        :disabled="disableButtons">
        Uninstall
      </button>
    </div>
    <div class="flex mt-4 justify-center items-center">{{ infoMessage }}</div>
  </div>
</template>

<style>
.button {
  padding: 16px;
  padding-right: 24px;
  padding-left: 24px;
  border-radius: 48px;
  font-weight: 500;
}

.button:hover,
button:focus:enabled {
  outline: none;
  opacity: 0.90;
}
</style>