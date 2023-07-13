<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { SearchOption, getSettings } from './Settings';
import InfoSVG from "../../assets/icons/info.svg";

const defaultSearchOptions = ref<SearchOption[]>();

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
    textColor:{
        required: true,
        type: String
    }
})

onMounted(async () => {
    let settings = await getSettings();
    defaultSearchOptions.value = settings.web_search.default;

})
</script>

<template>
    <div>
        <div class="mt-2 text-lg">Default </div>
        <div class="p-4 mt-2 rounded-lg flex secondaryBackground border items-center" v-for="option in defaultSearchOptions">
            <div class="accentText font-bold mr-2 ">{{ option.keyword }}</div>
            <div>{{ option.name }}</div>
            <div class="flex-grow"></div>
            <div>{{ option.query }}</div>
        </div>
        <div class="mt-2 text-lg">Custom</div>
        <button class="accentText font-bold hover:underline">Add</button>
        <div class=" p-4 justify-center flex items-center border secondaryBackground rounded-lg">
            <InfoSVG class="h-5 w-5 stroke mr-2" />
            <div>No custom searches</div>
        </div>
    </div>
</template>

<style scoped>
.accentText {
    color: v-bind(accentColor);
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.border{
    border: 1px solid v-bind(tertiaryBackgroundColor);
}

.stroke {
    fill: none;
    stroke: v-bind(textColor);
    stroke-width: 2
}
</style>