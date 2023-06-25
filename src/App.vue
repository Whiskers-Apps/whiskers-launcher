<script setup lang="ts">
import { appWindow, PhysicalSize } from "@tauri-apps/api/window"
import { move_window, Position } from "tauri-plugin-positioner-api"
import { invoke } from "@tauri-apps/api";
import { ref, watch } from "vue";

const searchText = ref("")


watch(searchText, async (oldSearch, newSearch) => {

})

function resizeWindow() {

  appWindow.setSize(new PhysicalSize(800, 140))
  move_window(Position.TopCenter)
  console.log("Entrou")
}
</script>

<template>
  <div class="flex items-center justify-center h-screen w-screen rounded-full bg-transparent">
    <input class="flex-grow p-2 rounded-full h-full" v-model="searchText" placeholder="Search For Apps">
  </div>
</template>

