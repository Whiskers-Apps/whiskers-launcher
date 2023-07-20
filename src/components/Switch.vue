<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getSettings } from '../pages/Settings/Settings';


defineProps({
    checked: {
        required: true,
        type: Boolean
    }
})

const backgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const accentColor = ref("");


const emit = defineEmits([
    "update:checked"
])

onMounted(async ()=>{
    let settings = await getSettings();
    backgroundColor.value = settings.theming.background;
    tertiaryBackgroundColor.value = settings.theming.tertiary_background;
    accentColor.value = settings.theming.accent;
})

</script>
<template>
    <label class="switch ml-2">
        <input type="checkbox" :checked="checked" @input="emit('update:checked', ($event.target as HTMLInputElement).checked)">
        <span class="switch round"></span>
    </label>
</template>
<style scoped>
.switch {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: v-bind(backgroundColor);
}

.switch:before {
    position: absolute;
    content: "";
    height: 20px;
    width: 20px;
    left: 4px;
    bottom: 1px;
    background-color: v-bind(tertiaryBackgroundColor);
    border: 1px solid v-bind(accentColor);
}


input:checked+.switch {
    background-color: v-bind(accentColor);
}

input:focus+.switch {
    box-shadow: 0 0 1px v-bind(accentColor);
}

input:checked+.switch:before {
    -webkit-transform: translateX(26px);
    -ms-transform: translateX(26px);
    transform: translateX(26px);
}

/* Rounded sliders */
.switch.round {
    border-radius: 34px;
}

.switch.round:before {
    border-radius: 50%;
}

</style>