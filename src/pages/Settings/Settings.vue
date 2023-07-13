<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { SettingsCategory, getSettings, updateSetting } from "./Settings"
import { invoke } from '@tauri-apps/api';
import GeneralTab from './GeneralTab.vue';
import ThemesTab from './ThemesTab.vue';
import SearchBoxTab from './SearchBoxTab.vue'
import WebSearchTab from './WebSearchTab.vue';
import ExtensionsTab from './ExtensionsTab.vue';

const currentCategory = ref(SettingsCategory.GENERAL)
const backgroundColor = ref()
const secondaryBackgroundColor = ref()
const tertiaryBackgroundColor = ref()
const accentColor = ref()
const onAccentColor = ref()
const textColor = ref()
const secondaryTextColor = ref("")

onMounted(async () => {
    let settings = await getSettings();

    backgroundColor.value = settings.theming.background;
    secondaryBackgroundColor.value = settings.theming.secondary_background;
    tertiaryBackgroundColor.value = settings.theming.tertiary_background;
    accentColor.value = settings.theming.accent;
    onAccentColor.value = settings.theming.on_accent;
    textColor.value = settings.theming.text;
    secondaryTextColor.value = settings.theming.seconday_text;
})

async function saveTheme() {

    var settings = await getSettings();

    settings.theming.background = backgroundColor.value;
    settings.theming.secondary_background = secondaryBackgroundColor.value;
    settings.theming.tertiary_background = tertiaryBackgroundColor.value;
    settings.theming.accent = accentColor.value;
    settings.theming.on_accent = onAccentColor.value;
    settings.theming.text = textColor.value;
    settings.theming.seconday_text = secondaryTextColor.value;

    invoke("update_settings", { settings_json: JSON.stringify(settings) });
}


</script>

<template>
    <div class=" h-screen w-screen max-h-screen max-w-screen background flex text">
        <div class=" w-56 h-screen secondaryBackground p-2 overflow-auto">
            <div class="tab" v-bind:class="currentCategory === SettingsCategory.GENERAL ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.GENERAL">General</div>
            <div class="tab mt-1" v-bind:class="currentCategory === SettingsCategory.SEARCH_BOX ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.SEARCH_BOX">Search Box</div>
            <div class="tab mt-1" v-bind:class="currentCategory === SettingsCategory.THEMING ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.THEMING">Theming</div>

            <div class="tab mt-1" v-bind:class="currentCategory === SettingsCategory.WEB_SEARCH ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.WEB_SEARCH">Web Search</div>

            <div class="tab mt-1" v-bind:class="currentCategory === SettingsCategory.EXTENSIONS ? 'activeTab' : ''"
                @click="currentCategory = SettingsCategory.EXTENSIONS">Extensions</div>
        </div>

        <div class="flex-grow p-2 overflow-auto">
            <div v-if="currentCategory === SettingsCategory.GENERAL">
                <GeneralTab :secondary-background-color="secondaryBackgroundColor" :accent-color="accentColor"
                    :text-color="textColor" :tertiary-background-color="tertiaryBackgroundColor" />
            </div>

            <div v-if="currentCategory === SettingsCategory.SEARCH_BOX">
                <SearchBoxTab :background-color="backgroundColor" :secondary-background-color="secondaryBackgroundColor"
                    :tertiary-background-color="tertiaryBackgroundColor" :accent-color="accentColor" />
            </div>

            <div v-if="currentCategory === SettingsCategory.THEMING">
                <ThemesTab :background-color="backgroundColor" @update:background-color="backgroundColor = $event.value"
                    :secondary-background-color="secondaryBackgroundColor"
                    @update:secondary-background-color="secondaryBackgroundColor = $event.value"
                    :tertiary-background-color="tertiaryBackgroundColor"
                    @update:tertiary-background-color="tertiaryBackgroundColor = $event.value" :accent-color="accentColor"
                    @update:accent-color="accentColor = $event.value" :text-color="textColor"
                    @update:text-color="textColor = $event.value" :on-accent-color="onAccentColor"
                    @update:on-accent-color="onAccentColor = $event.value" :secondary-text-color="secondaryTextColor"
                    @update:secondary-text-color="secondaryTextColor = $event.value" @save-theme="saveTheme()" />
            </div>
            <div v-if="currentCategory === SettingsCategory.WEB_SEARCH">
                <WebSearchTab :background-color="backgroundColor" :secondary-background-color="secondaryBackgroundColor"
                    :tertiary-background-color="tertiaryBackgroundColor" :accent-color="accentColor"
                    :text-color="textColor" />
            </div>
            <div v-if="currentCategory === SettingsCategory.EXTENSIONS">
                <ExtensionsTab :background-color="backgroundColor" :secondary-background-color="secondaryBackgroundColor"
                    :tertiary-background-color="tertiaryBackgroundColor" :accent-color="accentColor"
                    :text-color="textColor" />
            </div>
        </div>
    </div>
</template>


<style scoped>
.text {
    color: v-bind(textColor);
}

.background {
    background-color: v-bind(backgroundColor);
}

.secondaryBackground {
    background-color: v-bind(secondaryBackgroundColor);
}

.placeholder::placeholder {
    color: v-bind(secondaryTextColor);
}

::selection {
    background-color: v-bind(secondaryBackgroundColor);
}

.secondaryHover:hover {
    background-color: v-bind(secondaryBackgroundColor);
}

.tab {
    padding: 10px;
    border-radius: 10px;
    font-size: 17px;
}

.tab:hover {
    background-color: v-bind(tertiaryBackgroundColor);
    font-weight: 600;
    cursor: pointer;
}

.activeTab {
    background-color: v-bind(tertiaryBackgroundColor);
    font-weight: 600;
    
}

.stroke {
    fill: none;
    stroke: v-bind(accentColor);
    stroke-width: 2;
}
</style>