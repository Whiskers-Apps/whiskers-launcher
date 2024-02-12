<script setup lang="ts">
import PrimaryButton from "@components/PrimaryButton.vue";
import { ViewModel } from "./ViewModel";
import { onMounted, ref } from "vue";
import InputSetting from "@components/InputSetting.vue";
import SwitchSetting from "@components/SwitchSetting.vue";
import SelectSetting from "@components/SelectSetting.vue";
import SelectFileSetting from "@components/SelectFileSetting.vue";
import TextAreaSetting from "@components/TextAreaSetting.vue";

const vm = ref(new ViewModel());
const backgroundMain = ref("");
const accentPrimary = ref("");
const textOnBackground = ref("");

onMounted(async () => {
  await vm.value.load();

  const theme = vm.value.uiState.settings!!.theme;

  backgroundMain.value = theme.background_main;
  accentPrimary.value = theme.accent_primary;
  textOnBackground.value = theme.text_on_background;

  vm.value.uiState.hasLoaded = true;
});
</script>

<template>
  <div
    v-if="vm.uiState.hasLoaded"
    class="main h-screen max-h-screen p-4 flex flex-col"
  >
    <div class="overflow-auto flex-grow">
      <div v-for="field in vm.uiState.action!!.fields" class="mb-2">
        <div v-if="field.type === 'Input'">
          <InputSetting
            :title="field.title!!"
            :description="field.description ?? ''"
            :placeholder="field.placeholder"
            :value="vm.getFieldValue(field.id)"
            :theme="vm.uiState.settings!!.theme"
            @update-value="vm.updateFieldValue(field.id, $event)"
          />
        </div>
        <div v-if="field.type === 'TextArea'">
          <TextAreaSetting
            :title="field.title!!"
            :description="field.description ?? ''"
            :placeholder="field.placeholder"
            :value="vm.getFieldValue(field.id)"
            :theme="vm.uiState.settings!!.theme"
            @update-value="vm.updateFieldValue(field.id, $event)"
          />
        </div>
        <div v-if="field.type === 'Toggle'">
          <SwitchSetting
            :title="field.title!!"
            :description="field.description ?? ''"
            :checked="vm.getFieldValue(field.id) === 'true'"
            :theme="vm.uiState.settings!!.theme"
            @update-checked="vm.updateFieldValue(field.id, $event)"
          />
        </div>
        <div v-if="field.type === 'Select'">
          <SelectSetting
            :title="field.title!!"
            :description="field.description ?? ''"
            :options="vm.getSelectOptions(field)"
            :value="vm.getFieldValue(field.id)"
            :theme="vm.uiState.settings!!.theme"
            :use-secondary-background="true"
            @update-value="vm.updateFieldValue(field.id, $event)"
          />
        </div>
        <div v-if="field.type === 'SelectFile'">
          <SelectFileSetting
            :title="field.title!!"
            :description="field.description ?? ''"
            :value="vm.getFieldValue(field.id)"
            :select-dir="field.select_dir ?? false"
            :theme="vm.uiState.settings!!.theme"
            @on-click="vm.selectFile(field)"
          />
        </div>
      </div>
    </div>
    <div class="flex justify-end mt-4">
      <PrimaryButton
        :text="vm.uiState.action?.primary_button_text ?? 'Ok'"
        :theme="vm.uiState.settings!!.theme"
        @click="vm.closeDialog()"
      />
    </div>
  </div>
</template>

<style scoped>
::-webkit-scrollbar {
  width: 4px;
}

::-webkit-scrollbar-track {
  margin-top: 16px;
  margin-bottom: 16px;
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: v-bind(accentPrimary);
  border-radius: 48px;
}
.main {
  background-color: v-bind(backgroundMain);
  color: v-bind(textOnBackground);
}
</style>
