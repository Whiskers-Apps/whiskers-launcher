<script lang="ts">
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import { type Extension, type ExtensionSetting, type Settings } from '$lib/settings/settings';
	import { listen } from '@tauri-apps/api/event';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { createEventDispatcher, onMount } from 'svelte';
	import { WindowSizes } from '../../../utils';
	import { invoke } from '@tauri-apps/api';
	import Input from '$lib/components/input.svelte';
	import Select from '$lib/components/select.svelte';

	export let settings: Settings;
	let dispatch = createEventDispatcher();
	$: extensionsSettings = settings.extensions;
	let extensions: Extension[] = [];
	let os = '';

	onMount(async () => {
		os = await invoke('get_os');
		extensions = await invoke('get_extensions');
	});

	async function openCloneDialog() {
		new WebviewWindow('clone-extension', {
			url: 'dialogs/clone-extension',
			title: 'Clone Extension',
			height: WindowSizes.CloneExtension.height,
			width: WindowSizes.CloneExtension.width,
			resizable: false,
			maximizable: false
		});

		const unlisten = await listen('refresh-extensions', async () => {
			dispatch('refresh-extensions');
			unlisten();
		});
	}

	function getSettingValue(extensionId: string, settingId: string): string {
		for (const index in extensionsSettings) {
			if (
				extensionsSettings[index].extension_id === extensionId &&
				extensionsSettings[index].setting_id === settingId
			) {
				return extensionsSettings[index].setting_value;
			}
		}

		return '';
	}

	function canShowSetting(extensionId: string, setting: ExtensionSetting): boolean {

		if(setting.os !== "*" && setting.os.toLowerCase() != os){
			return false;
		}	

		if (setting.show_conditions !== null) {
			for (const index in setting.show_conditions) {
				const condition = setting.show_conditions[index];
				console.log(condition)
				console.log(getSettingValue(extensionId, condition.setting_id))

				if (condition.setting_value !== getSettingValue(extensionId, condition.setting_id)) {
					return false;
				}
			}
		}

		return true;
	}

	async function updateSetting(extensionId: string, settingId: string, value: string) {
		let newExtensionsSettings: ExtensionSetting[] = [];

		extensionsSettings.forEach((extensionSetting) => {
			if (
				extensionSetting.extension_id === extensionId &&
				extensionSetting.setting_id === settingId
			) {
				let setting = extensionSetting;
				setting.setting_value = value;
				newExtensionsSettings.push(setting);
			} else {
				newExtensionsSettings.push(extensionSetting);
			}
		});

		extensions = await invoke('get_extensions');

		dispatch('updateExtensionsSettings', newExtensionsSettings);
	}
</script>

<div class="flex">
	<SecondaryButton text="Extensions Store" />
	<SecondaryButton text="Git Clone" on:click={openCloneDialog} />
	<SecondaryButton text="Reload" />
</div>

{#each extensions as extension}
	<div class=" card">
		<div class=" flex">
			<p class=" flex-grow text-2xl font-bold">{extension.name}</p>
			<div>Open</div>
			<div>Uninstall</div>
		</div>
		<p class=" text-sub-text">{extension.description}</p>
		<div class=" v-divider mb-4 mt-4"></div>
		<div class=" space-y-4">
			{#if extension.settings !== null}
				{#each extension.settings as setting}
					{#if canShowSetting(extension.id, setting)}
						<div>
							<div class=" font-medium">{setting.title}</div>
							<div class=" text-sub-text">{setting.description}</div>

							{#if setting.setting_type === 'Input'}
								<Input
									value={getSettingValue(extension.id, setting.id)}
									on:input={(event) => {
										updateSetting(extension.id, setting.id, event.detail);
									}}
								/>
							{/if}
							{#if setting.setting_type === 'Select'}
								<Select
									values={setting.select_options ?? []}
									selectedValue={getSettingValue(extension.id, setting.id)}
									on:selection={(event) => {
										updateSetting(extension.id, setting.id, event.detail.id);
									}}
								/>
							{/if}
						</div>
					{/if}
				{/each}
			{/if}
		</div>
	</div>
{/each}
