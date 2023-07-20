<script setup lang="ts">
import { ref } from 'vue';


const emit = defineEmits([
    "update:backgroundColor",
    "update:secondaryBackgroundColor",
    "update:tertiaryBackgroundColor",
    "update:accentColor",
    "update:onAccentColor",
    "update:dangerColor",
    "update:onDangerColor",
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
    dangerColor: {
        required: true,
        type: String
    },
    onDangerColor: {
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
const currentDangerColor = ref(props.dangerColor);
const currentOnDangerColor = ref(props.onDangerColor);
const currentTextColor = ref(props.textColor);
const currentSecondaryTextColor = ref(props.secondaryTextColor);

function saveTheme() {

        emit("update:backgroundColor", currentBackgroundColor);
        emit("update:secondaryBackgroundColor", currentSecondaryBackgroundColor);
        emit("update:tertiaryBackgroundColor", currentTertiaryBackgroundColor);
        emit("update:accentColor", currentAccentColor);
        emit("update:onAccentColor", currentOnAccentColor);
        emit("update:dangerColor", currentDangerColor);
        emit("update:onDangerColor", currentOnDangerColor);
        emit("update:textColor", currentTextColor);
        emit("update:secondaryTextColor", currentSecondaryTextColor);
        emit("saveTheme");
}

</script>

<template>
    <div>
        <div class="text-lg font-bold">Import/Export</div>
        <div class="text-lg font-bold">Settings</div>
        <div class=" items-center secondaryBackground p-4 rounded-lg">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Background Color</div>
                <input v-model="currentBackgroundColor"
                    class="tertiaryBackground p-1 rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentBackground previewColor"></div>
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-4 rounded-lg">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Secondary Background Color</div>
                <input v-model="currentSecondaryBackgroundColor"
                    class="tertiaryBackground p-1 rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentSecondaryBackground previewColor"></div>
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-4 rounded-lg">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Tertiary Background Color</div>
                <input v-model="currentTertiaryBackgroundColor"
                    class="tertiaryBackground p-1 rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentTertiaryBackground previewColor"></div>
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-4 rounded-lg">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Accent Color</div>
                <input v-model="currentAccentColor"
                    class="tertiaryBackground p-1 rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentAccent previewColor"></div>
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-4 rounded-lg">
            <div class="flex items-center">
                <div class="flex-grow font-bold">On Accent Color</div>
                <input v-model="currentOnAccentColor"
                    class="tertiaryBackground p-1 rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentOnAccent previewColor"></div>
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-4 rounded-lg">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Danger Color</div>
                <input v-model="currentDangerColor"
                    class="tertiaryBackground p-1 rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentDanger previewColor"></div>
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-4 rounded-lg">
            <div class="flex items-center">
                <div class="flex-grow font-bold">On Danger Color</div>
                <input v-model="currentOnDangerColor"
                    class="tertiaryBackground p-1 rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentOnDanger previewColor"></div>
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-4 rounded-lg">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Text Color</div>
                <input v-model="currentTextColor" class="tertiaryBackground p-1 rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentText previewColor"></div>
            </div>
        </div>
        <div class=" mt-1 items-center secondaryBackground p-4 rounded-lg">
            <div class="flex items-center">
                <div class="flex-grow font-bold">Secondary Text Color</div>
                <input v-model="currentSecondaryTextColor"
                    class="tertiaryBackground p-1 rounded-lg outline-none w-20 flex text-center">
                <div class="ml-2 h-5 w-10 previewCurrentSecondaryText previewColor"></div>
            </div>
        </div>
        <div class="flex mt-4">
            <button class="flex-grow primaryButton rounded-xl pt-2 pb-2 pl-3 pr-3 hover:brightness-90 font-bold"
                @click="saveTheme()">Save Theme</button>
        </div>
    </div>
</template>

<style scoped>

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

.previewCurrentDanger {
    background-color: v-bind(currentDangerColor);
}

.previewCurrentOnDanger {
    background-color: v-bind(currentOnDangerColor);
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
    border: 1px solid v-bind(tertiaryBackgroundColor);
}

.primaryButton:hover{
    border-radius: 99px;
}
</style>