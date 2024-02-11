<script setup lang="ts">
import { ref } from 'vue';
import { Theme } from '@/pages/Settings/ViewModel';

const props = defineProps<{
  checked: boolean,
  theme: Theme,
  id?: string
}>();

const backgroundTertiary = ref(props.theme.background_tertiary)
const accentPrimary = ref(props.theme.accent_primary);


const emit = defineEmits([
  "update:checked"
])


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
  background-color: v-bind(backgroundTertiary);
  border: 2px solid v-bind(accentPrimary);
}

.slider:before {
  position: absolute;
  content: "";
  height: 22px;
  width: 22px;
  left: 4px;
  bottom: 2px;
  background-color: v-bind(backgroundTertiary);
  border: 2px solid v-bind(accentPrimary);
}

input:checked+.slider {
  background-color: v-bind(accentPrimary);
}

input:focus+.slider {
  box-shadow: 0 0 1px v-bind(accentPrimary);
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
