<script setup lang="ts">
import { ref } from "vue";
import { Extension, ExtensionSetting, ViewModel } from "./ViewModel";
import PrimaryButton from "@/components/PrimaryButton.vue";
import { SelectOption } from "@/components/ComponentClasses";
import InputField from "@/components/InputField.vue";
import SelectField from "@/components/SelectField.vue";
import Switch from "@/components/Switch.vue";
import { WebviewWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { CloneExtensionPayload } from "@/DialogPayloads";
import { event, invoke } from "@tauri-apps/api";

const props = defineProps<{
  vm: ViewModel;
}>();

const accentPrimary = ref(props.vm.settings!!.theme.accent_primary);
const accentDanger = ref(props.vm.settings!!.theme.accent_danger);

function getSettingValue(extensionId: string, settingId: string): string {
  let value = "";

  props.vm.settings!!.extensions.forEach((extension) => {
    if (extension.id === extensionId) {
      extension.settings.forEach((setting) => {
        if (setting.id === settingId) {
          value = setting.value;
        }
      });
    }
  });

  return value;
}

function getExtensionKeyword(extensionId: string): string {
  let keyword = "";

  props.vm.settings!!.extensions.forEach((extension) => {
    if (extension.id === extensionId) {
      keyword = extension.keyword;
    }
  });

  return keyword;
}

function getSelectOptions(extensionId: string, settingId: string): SelectOption[] {
  let selectOptions: SelectOption[] = [];

  props.vm.userExtensions.forEach((extension) => {
    if (extension.id === extensionId) {
      extension.settings.forEach((setting) => {
        if (setting.id === settingId) {
          setting.select_options?.forEach((option) => {
            selectOptions.push({
              id: option.id,
              text: option.text,
            });
          });
        }
      });
    }
  });

  return selectOptions;
}

async function updateExtensionSetting(extensionId: string, settingId: string, value: string) {
  let newExtensions: Extension[] = [];

  props.vm.settings!!.extensions.forEach((extension) => {
    if (extension.id === extensionId) {
      let newSettings: ExtensionSetting[] = [];

      extension.settings.forEach((setting) => {
        if (setting.id === settingId) {
          let newSetting = setting;
          newSetting.value = value;

          newSettings.push(newSetting);
        } else {
          newSettings.push(setting);
        }
      });

      newExtensions.push({
        id: extension.id,
        keyword: extension.keyword,
        settings: newSettings,
      });
    } else {
      newExtensions.push(extension);
    }
  });

  let newSettings = props.vm.settings!!;
  newSettings.extensions = newExtensions;

  props.vm.updateSettings(newSettings);
  props.vm.loadUserExtensions();
}

async function updateExtensionKeyword(extensionId: string, value: string) {
  let newExtensions: Extension[] = [];

  props.vm.settings!!.extensions.forEach((extension) => {
    if (extension.id === extensionId) {
      let newExtension = extension;
      newExtension.keyword = value;
      newExtensions.push(newExtension);
    } else {
      newExtensions.push(extension);
    }
  });

  let newSettings = props.vm.settings!!;
  newSettings.extensions = newExtensions;

  props.vm.updateSettings(newSettings);
  props.vm.loadUserExtensions();
}

function canShowSetting(extensionId: string, setting: ExtensionSetting): boolean {
  if (setting.show_conditions === null) {
    return true;
  }

  let canShowSetting = false;

  setting.show_conditions.forEach((condition) => {
    props.vm.settings!!.extensions.forEach((extension) => {
      if (extension.id === extensionId) {
        extension.settings?.forEach((setting) => {
          if (condition.setting_id === setting.id && condition.setting_value === setting.value) {
            canShowSetting = true;
          }
        });
      }
    });
  });

  return canShowSetting;
}

async function cloneExtension() {
  new WebviewWindow("clone-repo", {
    url: "clone-repo",
    width: 1000,
    height: 250,
    resizable: false,
    center: true,
    title: "Clone Extension",
  });

  const unlisten = await listen<CloneExtensionPayload>("clone-repo", async (event) => {
    await invoke("clone_extension", { url: event.payload.url });
    unlisten();

    props.vm.loadUserExtensions();
  });
}

async function uninstallExtension(extensionId: string) {
  new WebviewWindow("confirm-uninstall-extension", {
    url: "confirm-uninstall-extension",
    title: "Uninstall Extension",
    width: 1000,
    height: 200,
    resizable: false,
    center: true,
  });

  const unlisten = await listen("confirm-uninstall-extension", async (_event) => {
    await invoke("uninstall_extension", {
      extension_id: extensionId,
    });

    props.vm.loadUserExtensions();

    unlisten();
  });
}
</script>

<template>
  <div>
    <div class="flex">
      <PrimaryButton text="Reload" :theme="vm.settings!!.theme" @click="vm.loadUserExtensions()" />

      <PrimaryButton
        class="ml-2"
        text="Git Clone Extension"
        :theme="vm.settings!!.theme"
        @click="cloneExtension()"
      />

      <PrimaryButton class="ml-2" text="Extensions Store" :theme="vm.settings!!.theme" />
    </div>
    <div class="mt-4">
      <div v-for="extension in vm.userExtensions" class="p-4 background-secondary rounded-2xl mb-2">
        <div class="title">{{ extension.name }}</div>
        <div class="flex mb-4">
          <div class="version-card p-2 pr-3 pl-3">
            {{ extension.version_name }}
          </div>
          <button
            class="uninstall-button ml-2 p-2 pr-3 pl-3"
            @click="uninstallExtension(extension.id)"
          >
            Uninstall
          </button>
        </div>
        <div>
          <div class="text-primary font-medium text-xl">Keyword</div>
          <div>The extension keyword</div>
          <div class="mt-2 mb-2">
            <InputField
              :value="getExtensionKeyword(extension.id)"
              @on-change="updateExtensionKeyword(extension.id, $event)"
              :theme="vm.settings!!.theme"
              :use-background-tertiary="true"
            />
          </div>
        </div>
        <div v-for="setting in extension.settings" class="mb-1">
          <div v-if="canShowSetting(extension.id, setting)">
            <div v-if="setting.setting_type !== 'toggle'">
              <div class="text-primary font-medium text-xl">
                {{ setting.title }}
              </div>
              <div>
                {{ setting.description }}
              </div>
              <div class="mt-2" v-if="setting.setting_type === 'text'">
                <InputField
                  :value="getSettingValue(extension.id, setting.id)"
                  @on-change="updateExtensionSetting(extension.id, setting.id, $event)"
                  :theme="vm.settings!!.theme"
                  :use-background-tertiary="true"
                />
              </div>
              <div class="mt-2" v-if="setting.setting_type === 'select'">
                <SelectField
                  :value="getSettingValue(extension.id, setting.id)"
                  :options="getSelectOptions(extension.id, setting.id)"
                  :theme="vm.settings!!.theme"
                  @update-value="updateExtensionSetting(extension.id, setting.id, $event)"
                />
              </div>
            </div>
            <div v-else class="flex">
              <div class="mr-2 flex-grow">
                <div class="text-primary font-medium text-xl">
                  {{ setting.title }}
                </div>
                <div>
                  {{ setting.description }}
                </div>
              </div>
              <div>
                <Switch
                  class="mt-2"
                  :checked="getSettingValue(extension.id, setting.id) === 'true'"
                  :theme="vm.settings!!.theme"
                  @update:checked="
                    updateExtensionSetting(extension.id, setting.id, $event.toString())
                  "
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.version-card {
  color: v-bind(accentPrimary);
  border: 2px solid v-bind(accentPrimary);
  border-radius: 48px;
}

.update-button:focus {
  outline: 2px solid v-bind(accentPrimary);
}

.uninstall-button:focus {
  outline: 2px solid v-bind(accentDanger);
}

.uninstall-button {
  color: v-bind(accentDanger);
  border: 2px solid v-bind(accentDanger);
  border-radius: 48px;
}
</style>
