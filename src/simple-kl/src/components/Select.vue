<script setup lang="ts">
import { getTheme } from "@/pages/Settings/Settings";
import ChevronDownSVG from "@icons/chevron-down.svg";
import { PropType, onMounted, ref } from "vue";

const tertiaryBackgroundColor = ref("");
const textColor = ref("");

onMounted(async()=>{

    loadTheme();
});

async function loadTheme(){
    let theme = await getTheme();
    tertiaryBackgroundColor.value = theme.tertiary_background;
    textColor.value = theme.text;
}


export interface SelectOption{
    value: string,
    text: string
}

const props = defineProps({
    value:{
        required: true,
        type: String
    },
    options:{
        required: true,
        type: Object as PropType<SelectOption[]>
    },
    id:{
        required: false,
        type: String
    }
});

const emit = defineEmits([
    "updateValue"
])
</script>

<template>
    <div class="flex w-full selectBox">
        <select class="dropdown flex-grow" :id="id"
            :value="value"
            @change="event => emit('updateValue',(event.target as HTMLSelectElement).value)">
            <option v-for="option in options" :value="option.value" :key="option.value">
                <div>{{ option.text }}</div>
            </option>
        </select>
        <div class="flex items-center justify-center chevron">
            <ChevronDownSVG class="h-3 w-3 fill" />
        </div>
    </div>
</template>

<style scoped>
.selectBox {
    background-color: v-bind(tertiaryBackgroundColor);
    border: 1px solid v-bind(tertiaryBackgroundColor);
    border-radius: 48px;
}

.fill {
    fill: v-bind(textColor);
}

.chevron {
    padding: 8px;
    padding-right: 16px;
    border-top-right-radius: 48px;
    border-bottom-right-radius: 48px;
}

.dropdown {
    all: unset;
    display: flex;
    width: 100%;
    padding: 8px;
    padding-left: 16px;
    padding-right: 16px;
    cursor: pointer;
}
</style>