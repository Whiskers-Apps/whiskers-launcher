<script setup lang="ts">
import SelectField from "@components/SelectField.vue";
import { onMounted, ref } from "vue";
import { ViewModel } from "./ViewModel";
import { getHexCssFilter, getIconUrl } from "@/utils";
import SwitchSetting from "@/components/SwitchSetting.vue";
import SliderSetting from "@/components/SliderSetting.vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";

const props = defineProps<{ vm: ViewModel }>();
const onWayland = ref(false);
const hasLoaded = ref(false);

let accentPrimaryFilter = ref(getHexCssFilter(props.vm.settings!!.theme.accent_primary));

onMounted(async () => {
  await listen("load-theme", (_event) => {
    accentPrimaryFilter.value = getHexCssFilter(props.vm.settings!!.theme.accent_primary);
  });

  let os = await invoke("get_os");

  if (os == "linux") {
    let server: string = await invoke("get_display_server");
    onWayland.value = server.trim().toLowerCase() === "wayland";
  }

  hasLoaded.value = true;
});
</script>

<template>
  <div v-if="hasLoaded">
    <div class="title">Key Shortcut</div>
    <div>The shortcut to launch the search box</div>
    <div v-if="!onWayland">
      <div class="flex mt-2">
        <SelectField
          class="flex-grow"
          :value="vm.settings!!.launch_first_key"
          :options="vm.launchFirstKeyOptions"
          :theme="vm.settings!!.theme"
          @update-value="vm.updateLaunchFirstKey($event)"
        />

        <SelectField
          class="flex-grow ml-4 mr-4"
          :value="vm.settings!!.launch_second_key ? vm!!.settings!!.launch_second_key : '-'"
          :options="vm.launchSecondKeyOptions"
          :theme="vm.settings!!.theme"
          @update-value="vm.updateLaunchSecondKey($event)"
        />

        <SelectField
          class="flex-grow"
          :value="vm.settings!!.launch_third_key"
          :options="vm.launchThirdKeyOptions"
          :theme="vm.settings!!.theme"
          @update-value="vm.updateLaunchThirdKey($event)"
        />
      </div>

      <div class="flex mt-2" v-if="vm.launchShortcutChanged">
        <img :src="getIconUrl('warning.svg')" :style="{ filter: accentPrimaryFilter }" width="32" />
        <div class="ml-2">To apply this setting, please restart the companion app.</div>
      </div>
    </div>
    <div v-else>
      <div class="flex mt-2">
        <img :src="getIconUrl('warning.svg')" :style="{ filter: accentPrimaryFilter }" width="32" />
        <div class="ml-2 ">
          Wayland was detected! To add a shortcut you must launch
          <b>whiskers-launcher</b>
          binary using a shortcut. The way to add the shortcut will depend on the desktop
          environment/window manager
        </div>
      </div>
    </div>

    <div class="divider mt-2 mb-2"></div>

    <SwitchSetting
      title="Auto Start"
      description="If enabled, the app will autostart when the user logs in"
      :checked="vm.settings!!.auto_start"
      :theme="vm.settings!!.theme"
      @update-checked="vm.updateAutoStart($event)"
    />

    <div class="divider mt-2 mb-2"></div>

    <SliderSetting
      :title="`Fractional Scaling (${vm.settings!!.fractional_scaling.toFixed(2)})`"
      description="This feature is useful for windows users since the scaling is hard to deal with. It scales the pixels size."
      :value="vm.settings!!.fractional_scaling"
      :min="0.5"
      :max="2.0"
      :step="0.1"
      :theme="vm.settings!!.theme"
      @update-value="vm.updateFractionalScaling($event)"
    />

    <div class="divider mt-2 mb-2"></div>

    <SwitchSetting
      title="Hide On Blur"
      description="If enabled, it closes the search when the user clicks outside or the window loses focus"
      :checked="vm.settings!!.hide_on_blur"
      :theme="vm.settings!!.theme"
      @update-checked="vm.updateHideOnBlur($event)"
    />

    <SwitchSetting
      title="Show Recent Apps"
      description="If enabled, it shows the most recent apps when the search is empty"
      :checked="vm.settings!!.show_recent_apps"
      :theme="vm.settings!!.theme"
      @update-checked="vm.updateShowRecentApps($event)"
    />
  </div>
</template>
