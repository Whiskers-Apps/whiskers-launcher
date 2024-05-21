<script lang="ts">
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import { type Settings } from '$lib/settings/settings';
	import { listen } from '@tauri-apps/api/event';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { createEventDispatcher, onMount } from 'svelte';
	import { WindowSizes } from '../../../utils';

	export let settings: Settings;
    let dispatch = createEventDispatcher();
	$: extensionsSettings = settings.extensions;

	onMount(() => {});

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
            dispatch("refresh-extensions");
			unlisten();
		});
	}
</script>

<div class="flex">
	<SecondaryButton text="Extensions Store" />
	<SecondaryButton text="Git Clone" on:click={openCloneDialog} />
	<SecondaryButton text="Reload" />
</div>
