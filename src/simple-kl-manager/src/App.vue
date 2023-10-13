<script setup lang="ts">
import Logo from "@images/icon.png";
import { invoke } from "@tauri-apps/api";
import axios from "axios";

function install() {

  axios.get("https://api.github.com/repos/lighttigerxiv/simple-keyboard-launcher/releases/latest", {
    headers: {
      "User-Agent": "Simple Keyboard Launcher Manager - Tauri"
    }
  }).then(response => {

    if (response.status === 200) {
      response.data.assets.forEach((asset: any) => {
        if (asset.name.includes("linux")) {

          invoke("install", { url: asset.browser_download_url });
        }
      });
    }

  }).catch(error => console.error(error));

}
</script>

<template>
  <div class="p-6 bg-background h-screen max-h-screen overflow-auto text-text flex flex-col justify-center">
    <div class="flex items-center justify-center">
      <img :src="Logo" class="h-14" />
      <div class="ml-4 text-3xl font-medium">Simple KL Manager</div>
    </div>
    <div class="flex mt-6">
      <button class="flex-grow button bg-accent text-on-accent" @click="install()">
        Install
      </button>
      <button class="flex-grow button bg-danger text-on-danger ml-6">
        Uninstall
      </button>
    </div>
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
button:focus {
  outline: none;
  opacity: 0.90;
}
</style>