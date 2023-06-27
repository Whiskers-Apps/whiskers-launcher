<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getSettings, updateSetting } from './Settings';
import { invoke } from '@tauri-apps/api';


defineProps({
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
})

const firstKey = ref("");
const secondKey = ref("");
const thirdKey = ref("");
const resultsLimit = ref<number>();
const showFirstKeyOptions = ref(false);
const showSecondKeyOptions = ref(false);
const showThirdKeyOptions = ref(false);
const firstKeyOptions = ["Ctrl", "Alt", "Super", "Shift"]
const secondKeyOptions = ["-", "Alt", "Shift", "Super"]
const thirdKeyOptions = ["Space", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",]

onMounted(async () => {
    let settings = await getSettings();

    firstKey.value = settings.general.first_key;
    secondKey.value = settings.general.second_key;
    thirdKey.value = settings.general.third_key;
    resultsLimit.value = settings.general.limit;

})

async function updateKey(key: number, value: string) {

    if (key == 1) {
        firstKey.value = value;
        updateSetting("general_first_key", value);
        showFirstKeyOptions.value = false;
    }
    if (key == 2) {
        secondKey.value = value;
        updateSetting("general_second_key", value);
        showSecondKeyOptions.value = false;
    }
    if (key == 3) {
        thirdKey.value = value;
        updateSetting("general_third_key", value);
        showThirdKeyOptions.value = false;
    }
}

async function showKeyOptions(key: 1 | 2 | 3) {

    showFirstKeyOptions.value = false;
    showSecondKeyOptions.value = false;
    showThirdKeyOptions.value = false;

    switch (key) {
        case 1: { showFirstKeyOptions.value = true; break }
        case 2: { showSecondKeyOptions.value = true; break }
        case 3: { showThirdKeyOptions.value = true; break }
    }
}
</script>

<template>
    <div>
        <div class=" font-semibold text-lg">Keybinding</div>
        <div class="">Key combination to launch the search box</div>

        <div class="grid grid-cols-11 gap-2 mt-2">

            <div class="col-span-3 secondaryBackground rounded-md p-1 flex items-center justify-center"
                @click="showKeyOptions(1)">
                {{ firstKey }}
            </div>
            <div class="col-span-1 flex items-center justify-center tertiaryBackground">+</div>
            <div class="col-span-3 secondaryBackground rounded-md p-1 flex items-center justify-center"
                @click="showKeyOptions(2)">
                {{ secondKey }}
            </div>
            <div class="col-span-1 flex items-center justify-center tertiaryBackground">+</div>
            <div class="col-span-3 secondaryBackground rounded-md p-1 flex items-center justify-center"
                @click="showKeyOptions(3)">
                {{ thirdKey }}
            </div>
        </div>

        <div v-if="showFirstKeyOptions" class="grid grid-cols-8 gap-2 mt-2">
            <div v-for="option in firstKeyOptions">
                <button class="p-2 w-full secondaryBackground flex justify-center rounded-md" @click="updateKey(1, option)">
                    {{ option }}
                </button>
            </div>
        </div>

        <div v-if="showSecondKeyOptions" class="grid grid-cols-8 gap-2 mt-2">
            <div v-for="option in secondKeyOptions">
                <button class="p-2 w-full secondaryBackground flex justify-center rounded-md" @click="updateKey(2, option)">
                    {{ option }}
                </button>
            </div>
        </div>

        <div v-if="showThirdKeyOptions" class="grid grid-cols-8 gap-2 mt-2">
            <div v-for="option in thirdKeyOptions">
                <button class="p-2 w-full secondaryBackground flex justify-center rounded-md" @click="updateKey(3, option)">
                    {{ option }}
                </button>
            </div>
        </div>

        <div class=" font-semibold mt-4 text-lg">Results Limit</div>
        <div class="">The amount of results to show</div>
        <div class=" flex mt-2">
            <input type="range" class="flex-grow" step="1" min="1" max="8" :value="resultsLimit"
                @input="resultsLimit = +($event.target as HTMLInputElement).value">
            <div class="ml-2  pl-3 pr-3 pt-1 pb-1 secondaryBackground rounded-md">
                <div>{{ resultsLimit }}</div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.tertiaryBackground {
    background-color: v-bind(tertiaryBackgroundColor);
}

input[type="range"]::-webkit-slider-runnable-track {
    background: v-bind(secondaryBackgroundColor);
    border-radius: 9999px;
    height: 1rem;
}

input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    margin-top: -2px;
    background-color: v-bind(accentColor);
    height: 1.2rem;
    width: 1.2rem;
    border-radius: 9999px;
}
</style>