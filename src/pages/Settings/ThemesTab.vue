<script setup lang="ts">
import { ref } from 'vue';


const emit = defineEmits([
    "update:backgroundColor",
    "update:secondaryBackgroundColor",
    "update:tertiaryBackgroundColor",
    "update:accentColor",
    "update:onAccentColor",
    "update:textColor",
    "update:secondaryTextColor",
    "saveTheme"
])

const props = defineProps({
    backgroundColor: {
        required: true,
        type: String
    },
    secondaryBackgroundColor: {
        required: true,
        type: String
    },
    tertiaryBackgroundColor: {
        required: true,
        type: String
    },
    accentColor: {
        required: true,
        type: String
    },
    onAccentColor: {
        required: true,
        type: String
    },
    textColor: {
        required: true,
        type: String
    },
    secondaryTextColor: {
        required: true,
        type: String
    }
})


const currentBackgroundColor = ref(props.backgroundColor);
const currentSecondaryBackgroundColor = ref(props.secondaryBackgroundColor);
const currentTertiaryBackgroundColor = ref(props.tertiaryBackgroundColor);
const currentAccentColor = ref(props.accentColor);
const currentOnAccentColor = ref(props.onAccentColor);
const currentTextColor = ref(props.textColor);
const currentSecondaryTextColor = ref(props.secondaryTextColor);

function saveTheme() {

        emit("update:backgroundColor", currentBackgroundColor);
        emit("update:secondaryBackgroundColor", currentSecondaryBackgroundColor);
        emit("update:tertiaryBackgroundColor", currentTertiaryBackgroundColor);
        emit("update:accentColor", currentAccentColor);
        emit("update:onAccentColor", currentOnAccentColor);
        emit("update:textColor", currentTextColor);
        emit("update:secondaryTextColor", currentSecondaryTextColor);
        emit("saveTheme");
}

</script>

<template>
    <div>
        <div class="text-lg font-bold">Import/Export</div>
        <div class="text-lg font-bold">Settings</div>
        <div class=" items-center tertiaryBackground p-2 rounded-lg card">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Background Color</div>
                <input v-model="currentBackgroundColor"
                    class="secondaryBackground rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentBackground previewColor"></div>
            </div>
        </div>
        <div class=" mt-2 items-center tertiaryBackground p-2 rounded-lg card">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Secondary Background Color</div>
                <input v-model="currentSecondaryBackgroundColor"
                    class="secondaryBackground rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentSecondaryBackground previewColor"></div>
            </div>
        </div>
        <div class=" mt-2 items-center tertiaryBackground p-2 rounded-lg card">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Tertiary Background Color</div>
                <input v-model="currentTertiaryBackgroundColor"
                    class="secondaryBackground rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentTertiaryBackground previewColor"></div>
            </div>
        </div>
        <div class=" mt-2 items-center tertiaryBackground p-2 rounded-lg card">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Accent Color</div>
                <input v-model="currentAccentColor"
                    class="secondaryBackground rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentAccent previewColor"></div>
            </div>
        </div>
        <div class=" mt-2 items-center tertiaryBackground p-2 rounded-lg card">
            <div class="flex items-center">
                <div class="flex-grow font-bold">On Accent Color</div>
                <input v-model="currentOnAccentColor"
                    class="secondaryBackground rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentOnAccent previewColor"></div>
            </div>
        </div>
        <div class=" mt-2 items-center tertiaryBackground p-2 rounded-lg card">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Text Color</div>
                <input v-model="currentTextColor" class="secondaryBackground rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentText previewColor"></div>
            </div>
        </div>
        <div class=" mt-2 items-center tertiaryBackground p-2 rounded-lg card">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Secondary Text Color</div>
                <input v-model="currentSecondaryTextColor"
                    class="secondaryBackground rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentSecondaryText previewColor"></div>
            </div>
        </div>
        <div class="flex justify-end mt-4">
            <button class="primaryButton rounded-full pt-1 pb-1 pl-2 pr-2 hover:brightness-90 font-bold"
                @click="saveTheme()">Save Theme</button>
        </div>
    </div>
</template>

<style scoped>
.card {
    border: 1px solid v-bind(secondaryBackgroundColor);
}

.previewColor {
    border: 1px solid v-bind(textColor);
    border-radius: 9999px;
}

.previewCurrentBackground {
    background-color: v-bind(currentBackgroundColor);
}

.previewCurrentSecondaryBackground {
    background-color: v-bind(currentSecondaryBackgroundColor);
}

.previewCurrentTertiaryBackground {
    background-color: v-bind(currentTertiaryBackgroundColor);
}

.previewCurrentAccent {
    background-color: v-bind(currentAccentColor);
}

.previewCurrentOnAccent {
    background-color: v-bind(currentOnAccentColor);
}

.previewCurrentText {
    background-color: v-bind(currentTextColor);
}

.previewCurrentSecondaryText {
    background-color: v-bind(currentSecondaryTextColor);
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.tertiaryBackground {
    background-color: v-bind(tertiaryBackgroundColor);
}

.primaryButton {
    background-color: v-bind(accentColor);
    color: v-bind(onAccentColor);
}
</style>