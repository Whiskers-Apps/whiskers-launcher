<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '../pages/Settings/Settings';
import { listen } from '@tauri-apps/api/event';

const updateThemeEmit = ref();

defineProps({
  checked: {
    required: true,
    type: Boolean
  },
  id:{
    type: String,
    default: null
  }
})

const backgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");


const emit = defineEmits([
  "update:checked"
])

onMounted(async () => {
  loadTheme();

  updateThemeEmit.value = listen("updateTheme", (_event) => {
    loadTheme();
  })
})


async function loadTheme() {
  let theme = await getTheme();
  backgroundColor.value = theme.background;
  tertiaryBackgroundColor.value = theme.tertiary_background;
  accentColor.value = theme.accent;
}

</script>
<template>
  <label class="switch ml-2">
    <input :id="id" type="checkbox" :checked="checked"
      @input="emit('update:checked', ($event.target as HTMLInputElement).checked)">
    <span class="slider round"></span>
  </label>
</template>
<style scoped>
.switch {
  position: relative;
  display: inline-block;
  width: 57px;
  height: 30px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: v-bind(backgroundColor);
  border: 2px solid v-bind(accentColor);
}

.slider:before {
  position: absolute;
  content: "";
  height: 22px;
  width: 22px;
  left: 4px;
  bottom: 2px;
  background-color: v-bind(backgroundColor);
  border: 2px solid v-bind(accentColor);
}

input:checked+.slider {
  background-color: v-bind(accentColor);
}

input:focus+.slider {
  box-shadow: 0 0 1px v-bind(accentColor);
}

input:checked+.slider:before {
  -webkit-transform: translateX(26px);
  -ms-transform: translateX(26px);
  transform: translateX(26px);
}

/* Rounded sliders */
.slider.round {
  border-radius: 34px;
}

.slider.round:before {
  border-radius: 50%;
}
</style>