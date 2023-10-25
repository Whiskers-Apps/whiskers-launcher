<script setup lang="ts">
import { getTheme } from "@/pages/Settings/Settings";
import ChevronDownSVG from "@icons/chevron-down.svg";
import { PropType, onMounted, ref, watch } from "vue";


const props = defineProps({
    value: {
        required: true,
        type: String
    },
    options: {
        required: true,
        type: Object as PropType<SelectOption[]>
    },
    id: {
        required: false,
        type: String
    }
});

const emit = defineEmits([
    "updateValue"
])

const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const textColor = ref("");
const accentColor = ref("");

const input = ref("");
const showOptions = ref(false);
const searchText = ref("");
const selectedIndex = ref(0);
const currentOptions = ref<SelectOption[]>([])

onMounted(async () => {

    loadTheme();

    input.value = props.options.find(option => option.value === props.value)?.text ?? '';
    searchText.value = input.value;

    selectedIndex.value = props.options.findIndex(it => it.value === props.value) ?? 0;

    currentOptions.value = props.options;
});

document.addEventListener('keydown', function (event) {

    if (event.key === "ArrowDown") {

        event.preventDefault();

        if (selectedIndex.value < currentOptions.value.length - 1) {
            selectedIndex.value += 1;
        }
    }

    if (event.key === "ArrowUp") {

        event.preventDefault();

        if (selectedIndex.value > 0) {
            selectedIndex.value -= 1;
        }
    }

    if (event.key === "Enter") {
        updateValue(currentOptions.value[selectedIndex.value])
    }
});

async function loadTheme() {
    let theme = await getTheme();
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    textColor.value = theme.text;
    accentColor.value = theme.accent;
}


export interface SelectOption {
    value: string,
    text: string
}


function updateValue(option: SelectOption) {

    if (option.value !== "") {
        emit('updateValue', option);
        showOptions.value = false;

        selectedIndex.value = currentOptions.value.findIndex(it => it.value === option.value) ?? 0;

        currentOptions.value = props.options;
        input.value = option.text;
    }
}

function filter() {

    showOptions.value = true;

    let newOptions = props.options.filter(it => it.text.toLowerCase().includes(input.value.toLowerCase()));

    if (newOptions.length === 0) {
        newOptions.push({
            value: "",
            text: "Couldn't find any value"
        });
    }

    currentOptions.value = newOptions;

    selectedIndex.value = 0;

}


</script>

<template>
    <div class="w-full  flex flex-col">
        <div class="w-full flex selectBox">
            <input v-model="input" class="flex-grow dropdown relative" @click="showOptions = !showOptions"
                v-on:input="filter()" />

            <div class="flex items-center justify-center chevron">
                <ChevronDownSVG class="h-3 w-3 fillText" />
            </div>
        </div>

        <div class=" flex  ">
            <div v-if="showOptions" class=" overflow-clip optionsDiv w-full">
                <div v-for="(option, index) in currentOptions" 
                    :class="index === selectedIndex ? 'selectedOption' : ''" :id="option.value">
                    <button  @click="updateValue(option);" class="w-full text-start option">
                        {{ option.text }}
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.selectBox {
    background-color: v-bind(tertiaryBackgroundColor);
    border: 1px solid v-bind(tertiaryBackgroundColor);
    border-radius: 48px;
    overflow: auto;
    padding: 8px 16px 8px 16px;
}

.selectBox:focus-within {
    outline: 1px solid v-bind(accentColor);
}

.dropdown {
    all: unset;
    display: flex;
    width: 100%;
    cursor: pointer;
}

.optionsDiv {
    margin-top: 16px;
    position: relative;
    background-color: v-bind(tertiaryBackgroundColor);
    border-radius: 16px;
    border: 1px solid v-bind(accentColor);
}

.option {
    text-align: start;
    flex-grow: 1;
    padding: 16px;
}

.option:hover {
    background-color: v-bind(secondaryBackgroundColor);
}

.selectedOption {
    background-color: v-bind(secondaryBackgroundColor);
}

.fillText{
    fill: v-bind(textColor);
}
</style>