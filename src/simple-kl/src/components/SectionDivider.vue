<script setup lang="ts">
import { getTheme } from '@/pages/Settings/Settings';
import { onMounted, ref } from 'vue';

const tertiaryBackgroundColor = ref("");

onMounted(async()=>{
    loadTheme();
});


async function loadTheme(){

    let theme = await getTheme();
    tertiaryBackgroundColor.value = theme.tertiary_background;
}
</script>

<template>
    <div class=" pr-2 pl-2" >
        <div class="sectionDivider"></div>
    </div>
</template>

<style>
.sectionDivider{
    height: 2px;
    background-color: v-bind(tertiaryBackgroundColor);
    width: 100%;
}
</style>