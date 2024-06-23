<script lang="ts">
	import Input from '$lib/components/input.svelte';
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import { getSettings, getThemeCss, type Settings } from '$lib/settings/settings';
	import { invoke } from '@tauri-apps/api';
	import { emit } from '@tauri-apps/api/event';
	import { appWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';

	let settings: Settings | null = null;
	let css = '';

	$: url = '';
	const expression =
		/[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)/gi;
	const urlRegex = new RegExp(expression);

	$: disableButton = !urlRegex.test(url);

	onMount(async () => {
		settings = await getSettings();
		css = getThemeCss(settings);
	});

	function updateUrl(event: CustomEvent<string>) {
		url = event.detail;
	}

	async function clone() {
		disableButton = true;
		await invoke('clone_extension', {url: url});
		disableButton = true;

		await emit('refresh-extensions');
		appWindow.close();
	}
</script>

{#if settings !== null}
	{@html css}
	<div class=" gap-4 bg-background h-screen w-screen text-text p-4 flex flex-col justify-center">
		<div>
			<div class="text-2xl font-bold">Clone Extension</div>
			<Input value={url} placeholder="Type the extension repo url" on:input={updateUrl} />
		</div>
		<div class=" flex justify-end">
			<PrimaryButton text="Clone" disabled={disableButton} on:click={clone} />
		</div>
	</div>
{/if}
