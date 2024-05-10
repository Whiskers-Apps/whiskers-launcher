<script setup lang="ts">
import { ViewModel } from './ViewModel';
import { getVersion } from '@tauri-apps/api/app';
import {open} from '@tauri-apps/api/shell'
import { onMounted, ref } from 'vue';

defineProps<{
    vm: ViewModel
}>();

const appVersion = ref("");

onMounted(async () => {
    appVersion.value = await getVersion();
});

</script>

<template>
    <div>
        <div class="background-secondary rounded-2xl p-4">
            <div class="flex">
                <div class="mr-2 flex-grow font-semibold">Version</div>
                <div>{{ appVersion }}</div>
            </div>

            <div class="divider mt-2 mb-2"></div>

            <div class="flex">
                <div class="mr-2 flex-grow font-semibold">Extensions</div>
                <div>{{vm.userExtensions.length}}</div>
            </div>

            <div class="divider mt-2 mb-2"></div>

             <div class="flex">
                <div class="mr-2 flex-grow font-semibold">Source Code</div>
                <div class="underline cursor-pointer" @click="open('https://github.com/lighttigerXIV/whiskers-launcher')">GitHub</div>
            </div>

        </div> 
    </div>
</template>
