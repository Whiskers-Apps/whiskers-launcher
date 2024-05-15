<script lang="ts">
	import { page } from '$app/stores';
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import { getSettings, getThemeCss, type Settings } from '$lib/settings/settings';
	import { invoke } from '@tauri-apps/api';
	import { emit } from '@tauri-apps/api/event';
	import { appWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';

	let id = decodeURIComponent($page.params.id);
	let settings: Settings | null = null;
	let css = '';

	onMount(async () => {
		settings = await getSettings();
		css = getThemeCss(settings);
	});

	async function removeApp() {
		await invoke('remove_from_blacklist', { id: id });
		await emit('refresh-blacklist');
        
		appWindow.close();
	}
</script>

{#if settings !== null}
	{@html css}
	<div
		class=" p-4 bg-background text-text overflow-auto flex flex-col justify-center h-screen w-full"
	>
		<p class=" text-2xl font-bold">Remove From Blacklist</p>
		<p>Are you sure you want to remove this from the blacklist?</p>
		<div class=" flex justify-end">
			<PrimaryButton text="Confirm" on:click={removeApp} />
		</div>
	</div>
{/if}
