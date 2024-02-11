<script setup lang="ts">
import SelectSetting from "@/components/SelectSetting.vue";
import { WebviewWindow } from "@tauri-apps/api/window";
import { onMounted, PropType } from "vue";
import { ViewModel, DENSITY_OPTIONS } from "./ViewModel";
import SliderSetting from "@/components/SliderSetting.vue";
import PrimaryButton from "@/components/PrimaryButton.vue";
import SecondaryButton from "@components/SecondaryButton.vue";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { getHexCssFilter, getIconUrl } from "@/utils";

const props = defineProps({
    vm: {
        required: true,
        type: Object as PropType<ViewModel>,
    },
});

onMounted(async () => {
    props.vm.loadBacklistedApps();
});

async function openAddToBlacklist() {
    new WebviewWindow("add-to-blacklist", {
        url: "add-to-blacklist",
        title: "Add To Blacklist",
        width: 900,
        resizable: false,
    });

    const unlisten = await listen("refresh-blacklist", async (_event) => {
        props.vm.loadBacklistedApps();
        unlisten();
    });
}
</script>

<template>
    <div>
        <SliderSetting :title="`Results Count (${vm.settings!!.results_count})`" description="The amount of results to show"
            :min="2" :max="8" :step="1" :value="vm.settings!!.results_count" :theme="vm.settings!!.theme"
            @update-value="vm.updateResultsCount($event)" />

        <div class="divider mt-2 mb-2"></div>

        <SelectSetting title="Density" description="The results density" :options="DENSITY_OPTIONS"
            :value="vm.settings!!.density" :theme="vm.settings!!.theme" @update-value="vm.updateDensity($event)" />

        <div class="divider mt-2 mb-2"></div>

        <div class="flex">
            <div class="flex-grow">
                <div class="title">Blacklist</div>
            </div>
            <PrimaryButton text="Add" :theme="vm.settings!!.theme" @click="openAddToBlacklist()" />
        </div>
        <div v-if="vm.settings!!.blacklist.length == 0">
            Blacklist is empty. Click add to add a app to filter it on app results
        </div>
        <div v-else class="mt-2">
            <div v-for="app in vm.blacklistedApps" class="flex mb-2 items-center" :key="app.exec_path">
                <img v-if="app.icon_path !== undefined" :src="app.icon_path
                        ? convertFileSrc(app.icon_path)
                        : getIconUrl('default-app-icon.svg')
                    " width="40" :style="{
        filter: app.icon_path
            ? 'none'
            : getHexCssFilter(props.vm.settings!!.theme.accent_primary),
    }" />
                <div class="flex-grow ml-2 mr-2">{{ app.name }}</div>
                <SecondaryButton text="Remove" :theme="vm.settings!!.theme"
                    @click="vm.removeFromBlacklist(app.exec_path)" />
            </div>
        </div>
    </div>
</template>
