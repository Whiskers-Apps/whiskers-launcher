<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '../Settings/Settings';
import { listen } from '@tauri-apps/api/event';
import { DialogField, DialogResult, DialogFieldResult, DialogAction } from '@/data';
import { invoke } from '@tauri-apps/api';
import ChevronDown from "@icons/chevron-down.svg"
import PrimaryButton from '@/components/PrimaryButton.vue';
import Switch from '@/components/Switch.vue';

const updateThemeListener = ref();
const backgroundColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const textColor = ref("");
const secondaryTextColor = ref("");
const accentColor = ref("");
const dialogAction = ref<DialogAction>();

onMounted(async () => {

    loadTheme();

    updateThemeListener.value = listen("update-theme", (_) => {
        loadTheme();
    });

    dialogAction.value = await invoke("get_dialog_action");
});


async function loadTheme() {

    let theme = await getTheme();
    backgroundColor.value = theme.background;
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    textColor.value = theme.text;
    secondaryTextColor.value = theme.secondary_text;
    accentColor.value = theme.accent;
}

function getSelectValue(fieldID: string): string {

    let field = dialogAction.value?.fields.find(field => field.id === fieldID);

    return field !== undefined ? String(field.default_value) : ""
}

function finish() {

    let results: DialogFieldResult[] = [];

    dialogAction.value!!.fields!!.forEach((field) => {

        if (field.type === "Input") {
            results.push({
                field_id: field.id,
                value: (document.getElementById(field.id) as HTMLInputElement).value
            })
        }

        if (field.type === "Select") {
            results.push({
                field_id: field.id,
                value: (document.getElementById(field.id) as HTMLSelectElement).value
            })
        }

        if (field.type === "Toggle") {
            results.push({
                field_id: field.id,
                value: String((document.getElementById(field.id) as HTMLInputElement).checked)
            })
        }
    })

    let dialogResult: DialogResult = {
        extension_id: dialogAction.value!!.extension_id,
        action: dialogAction.value!!.action,
        results: results
    }

    invoke("write_dialog_result", { result: dialogResult })
}

</script>
<template>
    <div class="p-4 main h-screen overflow-auto flex flex-col">
        <div class="flex-grow">
            <div v-for="field in dialogAction?.fields" class="field">
                <div v-if="field.type === 'Toggle'" class="flex">
                    <div class="flex-grow">
                        <div class="font-bold ml-2">{{ field.title }}</div>
                        <div class="ml-2">{{ field.description }}</div>
                    </div>
                    <div class=" items-center">
                        <Switch :id="field.id" :checked="Boolean(field.value)" />
                    </div>
                </div>
                <div v-else>
                    <div class="font-bold ml-2">{{ field.title }}</div>
                    <div class="ml-2">{{ field.description }}</div>

                    <input v-if="field.type === 'Input'" :id="field.id" class="input" :placeholder="field.placeholder">

                    <div v-if="field.type === 'Select'" class="select flex items-center">
                        <select class="select-input" :id="field.id" :value="getSelectValue(field.id)">
                            <option v-for="option in field.options" :value="option.id">{{ option.value }}</option>
                        </select>

                        <ChevronDown class="h-3 w-3 select-chevron" />
                    </div>
                </div>
            </div>
        </div>

        <PrimaryButton :text="dialogAction?.button_text ?? ''" :expand="true" @click="finish()" />
    </div>
</template>
<style scoped>
.main {
    background-color: v-bind(backgroundColor);
    color: v-bind(textColor);
}

.field {
    background-color: v-bind(secondaryBackgroundColor);
    margin-bottom: 8px;
    padding: 16px;
    border-radius: 24px;
}

.input {
    background-color: v-bind(tertiaryBackgroundColor);
    padding: 8px;
    padding-right: 16px;
    padding-left: 16px;
    width: 100%;
    border-radius: 48px;
    margin-top: 8px;
}

.input::placeholder {
    color: v-bind(secondaryTextColor);
}

.select {
    -webkit-appearance: none;
    appearance: none;
    border-radius: 48px;
    background-color: v-bind(tertiaryBackgroundColor);
    overflow: hidden;
}

.select-input {
    -webkit-appearance: none;
    appearance: none;
    background-color: v-bind(tertiaryBackgroundColor);
    width: 100%;
    outline: none;
    cursor: pointer;
    padding: 8px;
    padding-left: 16px;
    padding-right: 16px;
}

.select-chevron {
    margin-right: 16px;
    fill: v-bind(secondaryTextColor);
}
</style>