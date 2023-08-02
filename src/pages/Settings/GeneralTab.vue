<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getSettings, updateSettings } from './Settings';
import Slider from '../../components/Slider.vue';

defineProps({
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
})

const firstKey = ref("");
const secondKey = ref("");
const thirdKey = ref("");
const resultsLimit = ref<number>(-1);
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
        showFirstKeyOptions.value = false;
    }
    if (key == 2) {
        secondKey.value = value;
        showSecondKeyOptions.value = false;
    }
    if (key == 3) {
        thirdKey.value = value;
        showThirdKeyOptions.value = false;
    }

    let settings = await getSettings();
    settings.general.first_key = firstKey.value;
    settings.general.second_key = secondKey.value;
    settings.general.third_key = thirdKey.value;

    updateSettings(settings);
}

function toggleShowKey(key: 1 | 2 | 3) {

    if (key == 1) {
        showFirstKeyOptions.value = !showFirstKeyOptions.value;
        showSecondKeyOptions.value = false;
        showThirdKeyOptions.value = false;
    }

    if (key == 2) {
        showFirstKeyOptions.value = false;
        showSecondKeyOptions.value = !showSecondKeyOptions.value;
        showThirdKeyOptions.value = false;
    }

    if (key === 3) {
        showFirstKeyOptions.value = false;
        showSecondKeyOptions.value = false;
        showThirdKeyOptions.value = !showThirdKeyOptions.value;
    }
}

async function updateResultsLimit(value: number) {

    resultsLimit.value = value;

    let settings = await getSettings();
    settings.general.limit = value;

    updateSettings(settings);
}

</script>

<template>
    <div class="p-4">

        <div class="ml-3 text-3xl">General</div>

        <div class=" secondaryBackground p-4 border rounded-3xl mt-4">
            <div class=" font-semibold text-lg">Keybinding</div>
            <div class="">Key combination to launch the search box</div>

            <div class="grid grid-cols-11 gap-2 mt-2 ">

                <button class="col-span-3 tertiaryBackground rounded-full p-1 flex items-center justify-center"
                    @click="toggleShowKey(1)">
                    {{ firstKey }}
                </button>
                <div class="col-span-1 flex items-center justify-center ">+</div>
                <button class="col-span-3 tertiaryBackground rounded-full p-1 flex items-center justify-center"
                    @click="toggleShowKey(2)">
                    {{ secondKey }}
                </button>
                <div class="col-span-1 flex items-center justify-center ">+</div>
                <button class="col-span-3 tertiaryBackground rounded-full p-1 flex items-center justify-center"
                    @click="toggleShowKey(3)">
                    {{ thirdKey }}
                </button>
            </div>

            <div v-if="showFirstKeyOptions" class="grid grid-cols-8 gap-2 mt-2">
                <div v-for="option in firstKeyOptions">
                    <button class="p-2 w-full tertiaryBackground flex justify-center rounded-md key border"
                        @click="updateKey(1, option)">
                        {{ option }}
                    </button>
                </div>
            </div>

            <div v-if="showSecondKeyOptions" class="grid grid-cols-8 gap-2 mt-2">
                <div v-for="option in secondKeyOptions">
                    <button class="p-2 w-full tertiaryBackground flex justify-center rounded-md key border"
                        @click="updateKey(2, option)">
                        {{ option }}
                    </button>
                </div>
            </div>

            <div v-if="showThirdKeyOptions" class="grid grid-cols-8 gap-2 mt-2">
                <div v-for="option in thirdKeyOptions">
                    <button class="p-2 w-full tertiaryBackground flex justify-center rounded-md key border"
                        @click="updateKey(3, option)">
                        {{ option }}
                    </button>
                </div>
            </div>
        </div>

        <div class="p-4 secondaryBackground border rounded-3xl mt-1">
            <div class=" font-semibold text-lg">Results Limit ({{ resultsLimit }})</div>
            <div class="">The amount of results to show</div>
            <div class="flex mt-2">
                <Slider :min="2" :max="8" :step="1" :value="resultsLimit" @update:value="updateResultsLimit($event)" />
            </div>
        </div>
    </div>
</template>

<style scoped>
.border {
    border: 1px solid v-bind(tertiaryBackgroundColor);
}

.key:hover {
    background: v-bind(secondaryBackgroundColor);
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.tertiaryBackground {
    background-color: v-bind(tertiaryBackgroundColor);
}

input[type="range"]::-webkit-slider-runnable-track {
    background: v-bind(tertiaryBackgroundColor);
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