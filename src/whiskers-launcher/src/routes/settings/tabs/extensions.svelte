<script lang="ts">
	import FolderIcon from '$lib/icons/folder.svg?component';
	import TrashIcon from '$lib/icons/trash.svg?component';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import { type Extension, type ExtensionSetting, type Settings } from '$lib/settings/settings';
	import { listen } from '@tauri-apps/api/event';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { createEventDispatcher, onMount } from 'svelte';
	import { WindowSizes } from '../../../utils';
	import { invoke } from '@tauri-apps/api';
	import Input from '$lib/components/input.svelte';
	import Select from '$lib/components/select.svelte';
	import TextArea from '$lib/components/text-area.svelte';
	import Toggle from '$lib/components/toggle.svelte';

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
			await reloadExtensions();
			unlisten();
		});
	}

	async function reloadExtensions() {
		await invoke('index_extensions');
		dispatch('refresh-extensions');
		extensions = await invoke('get_extensions');
	}

	async function openStore() {
		new WebviewWindow('extensions-store', {
			url: 'dialogs/extensions-store',
			title: 'Extensions Store',
			height: WindowSizes.Store.height,
			width: WindowSizes.Store.width,
			resizable: false,
			maximizable: false
		});

		const unlisten = await listen('refresh-extensions', async () => {
			await reloadExtensions();
			unlisten();
		});
	}

	async function openExtensionDirectory(extensionId: string) {
		invoke('open_extension_dir', { id: extensionId });
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
		if (setting.os !== '*' && setting.os.toLowerCase() != os) {
			return false;
		}

		if (setting.show_conditions !== null) {
			for (const index in setting.show_conditions) {
				const condition = setting.show_conditions[index];

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

	async function openRemoveExtensionDialog(extensionId: string) {
		new WebviewWindow('remove-extension', {
			url: `dialogs/remove-extension?id=${encodeURIComponent(extensionId)}`,
			title: 'Remove Extension',
			height: WindowSizes.ConfirmDialog.height,
			width: WindowSizes.ConfirmDialog.width,
			resizable: false,
			maximizable: false
		});

		const unlisten = await listen('refresh-extensions', async () => {
			await reloadExtensions();
			unlisten();
		});
	}
</script>

<div class="flex">
	<SecondaryButton text="Extensions Store" on:click={openStore} />
	<SecondaryButton text="Git Clone" on:click={openCloneDialog} />
	<SecondaryButton text="Reload" on:click={reloadExtensions} />
</div>
<div class=" space-y-4">
	{#each extensions as extension}
		<div class=" card">
			<div class=" flex">
				<p class=" flex-grow text-2xl font-bold">{extension.name}</p>
				<button
					class=" p-2 hover-bg-tertiary text-accent rounded-full hover-text-on-accent hover-bg-accent"
					on:click={() => openExtensionDirectory(extension.id)}
				>
					<FolderIcon class=" " width="24" height="24" />
				</button>
				<button
					class=" p-2 hover-bg-tertiary text-danger rounded-full hover-text-on-danger hover-bg-danger"
					on:click={() => openRemoveExtensionDialog(extension.id)}
				>
					<TrashIcon class=" " width="24" height="24" />
				</button>
			</div>
			<p class=" text-sub-text">{extension.description}</p>
			<div class=" v-divider mb-4 mt-4"></div>
			<div class=" space-y-4">
				<div>
					<div class=" font-medium">Keyword</div>
					<div class=" text-sub-text">The extension keyword</div>
					<Input
						value={getSettingValue(extension.id, 'keyword')}
						on:input={(event) => {
							updateSetting(extension.id, 'keyword', event.detail);
						}}
					/>
				</div>
				{#if extension.settings !== null}
					{#each extension.settings as setting}
						{#if canShowSetting(extension.id, setting)}
							<div>
								{#if setting.setting_type === 'Input'}
									<div class=" font-medium">{setting.title}</div>
									<div class=" text-sub-text">{setting.description}</div>

									<Input
										value={getSettingValue(extension.id, setting.id)}
										on:input={(event) => {
											updateSetting(extension.id, setting.id, event.detail);
										}}
									/>
								{/if}
								{#if setting.setting_type === 'TextArea'}
									<div class=" font-medium">{setting.title}</div>
									<div class=" text-sub-text">{setting.description}</div>

									<TextArea
										value={getSettingValue(extension.id, setting.id)}
										on:input={(event) => {
											updateSetting(extension.id, setting.id, event.detail);
										}}
									/>
								{/if}
								{#if setting.setting_type === 'Select'}
									<div class=" font-medium">{setting.title}</div>
									<div class=" text-sub-text">{setting.description}</div>

									<Select
										values={setting.select_options ?? []}
										selectedValue={getSettingValue(extension.id, setting.id)}
										on:selection={(event) => {
											updateSetting(extension.id, setting.id, event.detail.id);
										}}
									/>
								{/if}
								{#if setting.setting_type === 'Toggle'}
									<div class="flex">
										<div class="flex-grow">
											<div class=" font-medium">{setting.title}</div>
											<div class=" text-sub-text">{setting.description}</div>
										</div>
										<Toggle
											toggled={getSettingValue(extension.id, setting.id) === 'true' ? true : false}
											on:toggle={(e) => {
												updateSetting(extension.id, setting.id, String(e.detail));
											}}
										/>
									</div>
								{/if}
							</div>
						{/if}
					{/each}
				{/if}
			</div>
		</div>
	{/each}
</div>
