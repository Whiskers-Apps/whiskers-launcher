<script lang="ts">
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import { type Extension, type Settings } from '$lib/settings/settings';
	import { listen } from '@tauri-apps/api/event';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { createEventDispatcher, onMount } from 'svelte';
	import { WindowSizes } from '../../../utils';
	import { invoke } from '@tauri-apps/api';
	import Input from '$lib/components/input.svelte';

	export let settings: Settings;
	let dispatch = createEventDispatcher();
	$: extensionsSettings = settings.extensions;
	let extensions: Extension[] = [];

	onMount(async () => {
		extensions = await invoke('get_extensions');
		console.log(JSON.stringify(extensions));
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
					<div>
						<div class=" font-medium">{setting.title}</div>
						<div class=" text-sub-text">{setting.description}</div>

						{#if setting.setting_type === "Input"}
							<Input value={setting.default_value}/>
						{/if}
					</div>
				{/each}
			{/if}
		</div>
	</div>
{/each}
